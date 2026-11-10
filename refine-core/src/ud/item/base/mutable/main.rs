use crate::{
    EffectMode, ItemId, UnitInterval, Value,
    ad::{AAttrId, AEffectId, AItemId},
    err::basic::ItemNotMutatedError,
    rd::{
        RAttrId, RData, REffectId, RItemBase, RItemEffectOptions, RItemFlexData, RMuta, RMutaAttrRange,
        RMutasPregenMergedAttrsId, RState, RcMuta,
    },
    ud::{
        UAttrMutationRequest, UItemMutationRequest,
        item::{
            REffs, UEffectModeOverrideIter,
            base::{
                UEffectUpdates, UItemBase,
                mutable::err::{ItemMutatedError, MutatorInplaceError},
            },
        },
    },
    util::RMap,
};

/// Item mutable base stores all the data every mutable item should have.
///
/// Mutated item can have 3 states:
/// - Non-mutated - mutation is not set, only base item info is used
/// - Mutated, mutation is loaded - source had all the needed mutation data, which was processed and
///   stored on cache. In this case, item base stores mutated item type, and base aitem ID is stored
///   on mutation cache;
/// - Mutated, mutation not loaded - item base stores base item type, mutation stores mutator ID and
///   attribute mutations, and mutation cache isn't set.
///
/// Mutated items expose RItemBase of mutated item, and RItemAttrData which contains "hybrid" data:
/// - Attribute values - merged attributes (base item attrs + mutated item attrs) with all the
///   mutations applied;
/// - Everything else (per-effect attr-derived data, on-item attr-derived data) is built using
///   merged attributes, but not modified by mutations (it also means mutating attributes does not
///   change this part). Those properties are stored here for performance reasons, and are supposed
///   to be "immutable". Them being unchangeable by mutations is intentional. If there was need to
///   use mutated/modified value, those properties could be recalculated by request using attribute
///   values fetched from calc.
#[derive(Clone)]
pub(in crate::ud::item) struct UItemBaseMutable {
    base: UItemBase,
    mutation: Option<ItemMutationData>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constructor
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemBaseMutable {
    pub(in crate::ud::item) fn new(
        item_id: ItemId,
        type_aid: AItemId,
        state: RState,
        mutation_request: Option<UItemMutationRequest>,
        r_data: &RData,
    ) -> Self {
        let Some(mutation_request) = mutation_request else {
            // No mutation - regular non-mutated item setup
            return Self {
                base: UItemBase::new(item_id, type_aid, state, r_data),
                mutation: None,
            };
        };
        let mutator_type_aid = mutation_request.mutator_type_aid;
        let mut item_mutation_data = convert_request_to_data(mutation_request);
        let Some(mutator) = r_data.get_mutator_by_aid(&mutator_type_aid) else {
            // No mutator - base item with ineffective user-defined mutations
            return Self {
                base: UItemBase::new(item_id, type_aid, state, r_data),
                mutation: Some(item_mutation_data),
            };
        };
        let muta_pregen = r_data.get_muta_pregen();
        let Some(pregen_entry) = muta_pregen.get_entry(type_aid, mutator_type_aid) else {
            // No mutated item ID in mapping or no mutated item itself
            return match r_data.get_item_by_aid(&type_aid) {
                // If base item is available, return base item, but with ineffective
                // user-defined mutations
                Some(base_r_item) => Self {
                    base: UItemBase::base_with_r_item(item_id, base_r_item.clone(), state),
                    mutation: Some(item_mutation_data),
                },
                // No base item - unloaded item with ineffective user-defined mutations
                None => Self {
                    base: UItemBase::base_with_type_aid_not_loaded(item_id, type_aid, state),
                    mutation: Some(item_mutation_data),
                },
            };
        };
        // Make proper mutated item once we have all the data
        let mut item_flex_data = muta_pregen.get_unmutated_flex_data(pregen_entry);
        apply_attr_mutations(&mut item_flex_data, mutator, &item_mutation_data.attr_rolls, r_data);
        let regular_base = UItemBase::base_with_r_item(item_id, pregen_entry.mutated_item.clone(), state);
        item_mutation_data.cache = Some(ItemMutationDataCache {
            base_type_aid: type_aid,
            pregen_attrs_rid: pregen_entry.merged_attrs_rid,
            mutator: mutator.clone(),
            flex_data: item_flex_data,
        });
        Self {
            base: regular_base,
            mutation: Some(item_mutation_data),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Runtime data methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemBaseMutable {
    pub(in crate::ud::item) fn get_r_item_base(&self) -> Option<&RItemBase> {
        self.base.get_r_item_base()
    }
    pub(in crate::ud::item) fn get_r_item_flex_data(&self) -> Option<&RItemFlexData> {
        let Some(item_mutation) = &self.mutation else {
            return self.base.get_r_item_flex_data();
        };
        match &item_mutation.cache {
            Some(cache) => Some(&cache.flex_data),
            None => self.base.get_r_item_flex_data(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// User data methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemBaseMutable {
    pub(in crate::ud::item) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(in crate::ud::item) fn get_base_type_aid(&self) -> AItemId {
        match self.mutation.as_ref().and_then(|v| v.cache.as_ref()) {
            Some(cache) => cache.base_type_aid,
            None => self.base.get_type_aid(),
        }
    }
    pub(in crate::ud::item) fn set_base_type_aid(&mut self, type_aid: AItemId, r_data: &RData) {
        // Since this method is supposed to update base item ID for mutated items, location of ID
        // depends on item configuration
        match &mut self.mutation {
            Some(mutation_data) => match &mut mutation_data.cache {
                Some(mutation_cache) => {
                    mutation_cache.base_type_aid = type_aid;
                }
                None => self.base.base_set_type_aid_primitive(type_aid),
            },
            None => self.base.base_set_type_aid_primitive(type_aid),
        }
        // Even if mutation is not effective with old base type ID, it might become effective with
        // the new one, so - update the data the mutated way regardless of presence of the mutation
        // cache
        self.update_r_data(r_data);
    }
    /// Attempt to set base type, with the condition that rendered item type and data derived from
    /// attributes do not change. Returns vector with IDs of attributes whose values changed if base
    /// type was updated.
    pub(in crate::ud::item) fn try_set_base_type_aid_inplace(
        &mut self,
        type_aid: AItemId,
        r_data: &RData,
    ) -> Option<Vec<RAttrId>> {
        // Only effective mutation can keep rendered item type when base type changes
        let item_mutation = self.mutation.as_mut()?;
        let mutation_cache = item_mutation.cache.as_mut()?;
        let muta_pregen = r_data.get_muta_pregen();
        let new_pregen_entry = muta_pregen.get_entry(type_aid, item_mutation.mutator_type_aid)?;
        // With mutation cache set, item base stores mutated item
        let mutated_r_item = self.base.base_get_r_item().unwrap();
        if new_pregen_entry.mutated_item.base.aid != mutated_r_item.base.aid {
            return None;
        }
        // Same unmutated attributes mean the same data derived from them, and the same attribute
        // values
        if new_pregen_entry.merged_attrs_rid == mutation_cache.pregen_attrs_rid {
            mutation_cache.base_type_aid = type_aid;
            return Some(Vec::new());
        }
        // Services keep data derived from attributes, so with it changed, item has to be re-added.
        // Mutation cache is set only when pregenerated data exists
        let old_pregen_entry = muta_pregen
            .get_entry(mutation_cache.base_type_aid, item_mutation.mutator_type_aid)
            .unwrap();
        if !muta_pregen.is_same_flex_data(old_pregen_entry, new_pregen_entry) {
            return None;
        }
        let new_attrs = muta_pregen.get_merged_attrs(new_pregen_entry.merged_attrs_rid).clone();
        let old_attrs = std::mem::replace(&mut mutation_cache.flex_data.attrs, new_attrs);
        apply_attr_mutations(
            &mut mutation_cache.flex_data,
            &mutation_cache.mutator,
            &item_mutation.attr_rolls,
            r_data,
        );
        let new_attrs = &mutation_cache.flex_data.attrs;
        // Presized, since growing it costs more than unused capacity
        let mut changed_attr_rids = Vec::with_capacity(new_attrs.len());
        let mut old_found = 0;
        for (&attr_rid, new_value) in new_attrs.iter() {
            match old_attrs.get(&attr_rid) {
                Some(old_value) => {
                    old_found += 1;
                    if old_value != new_value {
                        changed_attr_rids.push(attr_rid);
                    }
                }
                None => changed_attr_rids.push(attr_rid),
            }
        }
        // Attributes which are gone have to be looked for only if some old ones were not found
        if old_found != old_attrs.len() {
            for &attr_rid in old_attrs.keys() {
                if !new_attrs.contains_key(&attr_rid) {
                    changed_attr_rids.push(attr_rid);
                }
            }
        }
        mutation_cache.base_type_aid = type_aid;
        mutation_cache.pregen_attrs_rid = new_pregen_entry.merged_attrs_rid;
        Some(changed_attr_rids)
    }
    pub(in crate::ud::item) fn get_rendered_type_aid(&self) -> AItemId {
        self.base.get_type_aid()
    }
    pub(in crate::ud::item) fn get_state(&self) -> RState {
        self.base.get_state()
    }
    pub(in crate::ud::item) fn set_state(&mut self, state: RState) {
        self.base.set_state(state)
    }
    pub(in crate::ud::item) fn get_effect_mode_by_rid(&self, effect_rid: REffectId) -> EffectMode {
        self.base.get_effect_mode_by_rid(effect_rid)
    }
    pub(in crate::ud::item) fn iter_effect_mode_overrides(&self) -> UEffectModeOverrideIter<'_> {
        self.base.iter_effect_mode_overrides()
    }
    pub(in crate::ud::item) fn set_effect_mode(
        &mut self,
        effect_aid: AEffectId,
        effect_mode: EffectMode,
        r_data: &RData,
    ) -> EffectMode {
        self.base.set_effect_mode(effect_aid, effect_mode, r_data)
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        r_data: &RData,
    ) -> bool {
        self.base.set_effect_modes(modes, r_data)
    }
    pub(in crate::ud::item) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::ud::item) fn r_data_changed(&mut self, r_data: &RData) {
        self.base.base_update_effect_modes(r_data);
        self.update_r_data(r_data);
    }
    fn update_r_data(&mut self, r_data: &RData) {
        let Some(item_mutation) = &mut self.mutation else {
            // No mutation - just update base item
            self.base.base_update_r_data(r_data);
            return;
        };
        let base_type_aid = match &item_mutation.cache {
            Some(cache) => cache.base_type_aid,
            None => self.base.get_type_aid(),
        };
        let Some(mutator) = r_data.get_mutator_by_aid(&item_mutation.mutator_type_aid) else {
            // No mutator - invalidate mutated cache and use non-mutated item
            match r_data.get_item_by_aid(&base_type_aid) {
                Some(base_r_item) => {
                    self.base.base_set_r_item(base_r_item.clone());
                    item_mutation.cache = None;
                    return;
                }
                None => {
                    self.base.base_set_type_aid_not_loaded(base_type_aid);
                    item_mutation.cache = None;
                    return;
                }
            }
        };
        let muta_pregen = r_data.get_muta_pregen();
        let Some(pregen_entry) = muta_pregen.get_entry(base_type_aid, item_mutation.mutator_type_aid) else {
            // No mutated aitem ID or no item itself - invalidate mutated cache and use non-mutated
            // item
            match r_data.get_item_by_aid(&base_type_aid) {
                Some(base_r_item) => {
                    self.base.base_set_r_item(base_r_item.clone());
                    item_mutation.cache = None;
                    return;
                }
                None => {
                    self.base.base_set_type_aid_not_loaded(base_type_aid);
                    item_mutation.cache = None;
                    return;
                }
            }
        };
        // Compose attribute cache
        let mut item_flex_data = muta_pregen.get_unmutated_flex_data(pregen_entry);
        apply_attr_mutations(&mut item_flex_data, mutator, &item_mutation.attr_rolls, r_data);
        // Everything needed is at hand, update item
        self.base.base_set_r_item(pregen_entry.mutated_item.clone());
        item_mutation.cache = Some(ItemMutationDataCache {
            base_type_aid,
            pregen_attrs_rid: pregen_entry.merged_attrs_rid,
            mutator: mutator.clone(),
            flex_data: item_flex_data,
        });
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mutation-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemBaseMutable {
    pub(in crate::ud::item) fn get_mutation_data(&self) -> Option<&ItemMutationData> {
        self.mutation.as_ref()
    }
    pub(in crate::ud::item) fn get_mutator_type_aid(&self) -> Option<AItemId> {
        self.mutation.as_ref().map(|v| v.mutator_type_aid)
    }
    pub(in crate::ud::item) fn mutate(
        &mut self,
        mutation_request: UItemMutationRequest,
        r_data: &RData,
    ) -> Result<(), ItemNotMutatedError> {
        if self.get_mutation_data().is_some() {
            return Err(ItemNotMutatedError {
                item_id: self.get_item_id(),
            });
        };
        // Since item is not mutated, base aitem ID is always on non-mutated item base
        let base_type_aid = self.base.get_type_aid();
        let mutator_type_aid = mutation_request.mutator_type_aid;
        let mut item_mutation_data = convert_request_to_data(mutation_request);
        let Some(mutator) = r_data.get_mutator_by_aid(&mutator_type_aid) else {
            // No mutator - nothing changes, except for user-defined mutations getting stored
            self.mutation = Some(item_mutation_data);
            return Ok(());
        };
        let muta_pregen = r_data.get_muta_pregen();
        let Some(pregen_entry) = muta_pregen.get_entry(base_type_aid, mutator_type_aid) else {
            // No mutated aitem ID or no mutated item itself - nothing changes, except for
            // user-defined mutations getting stored
            self.mutation = Some(item_mutation_data);
            return Ok(());
        };
        // Since we have all the data now, apply mutation properly
        let mut item_flex_data = muta_pregen.get_unmutated_flex_data(pregen_entry);
        apply_attr_mutations(&mut item_flex_data, mutator, &item_mutation_data.attr_rolls, r_data);
        self.base.base_set_r_item(pregen_entry.mutated_item.clone());
        item_mutation_data.cache = Some(ItemMutationDataCache {
            base_type_aid,
            pregen_attrs_rid: pregen_entry.merged_attrs_rid,
            mutator: mutator.clone(),
            flex_data: item_flex_data,
        });
        self.mutation = Some(item_mutation_data);
        Ok(())
    }
    /// Applies attribute mutation request, returning attribute ID if its value changed.
    ///
    /// Roll in the request is replaced with the roll item had, which later can be used to undo it.
    pub(in crate::ud::item) fn change_mutation_attr(
        &mut self,
        r_data: &RData,
        attr_mutation_request: &mut UAttrMutationRequest,
    ) -> Result<Option<RAttrId>, ItemMutatedError> {
        let Some(item_mutation) = &mut self.mutation else {
            return Err(ItemMutatedError);
        };
        let new_roll = attr_mutation_request.roll;
        // Update user-defined data
        attr_mutation_request.roll = match new_roll {
            Some(roll) => item_mutation.attr_rolls.insert(attr_mutation_request.attr_aid, roll),
            None => item_mutation.attr_rolls.remove(&attr_mutation_request.attr_aid),
        };
        let Some(mutation_cache) = &mut item_mutation.cache else {
            // If there is no cache - mutations are not effective, and none of item attributes can
            // change
            return Ok(None);
        };
        // Pregenerated attributes ID is valid for the source the cache was generated with, and this
        // method is supposed to be called with the same source
        let unmutated_attrs = r_data
            .get_muta_pregen()
            .get_merged_attrs(mutation_cache.pregen_attrs_rid);
        // No unmutated value now means there couldn't be any mutated value with any mutation
        // earlier as well, thus attribute value cannot change
        let Some(attr_rid) = r_data.get_attr_rid_by_aid(&attr_mutation_request.attr_aid) else {
            return Ok(None);
        };
        let Some(&unmutated_value) = unmutated_attrs.get(&attr_rid) else {
            return Ok(None);
        };
        let Some(mutation_range) = mutation_cache.mutator.attr_mods.get(&attr_rid) else {
            // No mutation range now means there couldn't be any mutated value earlier as well,
            // regardless of user-defined roll data, thus attribute value cannot change
            return Ok(None);
        };
        let new_value = mutate_attr_value(unmutated_value, mutation_range, new_roll);
        // Since unmutated value of the attribute is available by now, we can safely assume that
        // merged attributes have some value too (those are supposed to be built using the same
        // logic as unmutated value)
        let old_value = mutation_cache.flex_data.attrs.insert(attr_rid, new_value).unwrap();
        match old_value != new_value {
            true => Ok(Some(attr_rid)),
            false => Ok(None),
        }
    }
    /// Applies attribute mutation requests, returning IDs of attributes whose values changed. A
    /// batch version of single-request method.
    ///
    /// Requests are replaced with the values item had, which later can be used to undo them.
    pub(in crate::ud::item) fn change_mutation_attrs(
        &mut self,
        r_data: &RData,
        attr_mutation_requests: &mut [UAttrMutationRequest],
    ) -> Result<Vec<RAttrId>, ItemMutatedError> {
        let Some(item_mutation) = &mut self.mutation else {
            return Err(ItemMutatedError);
        };
        let Some(mutation_cache) = &mut item_mutation.cache else {
            // If there is no cache - mutations are not effective. In this case we update user data
            // and return empty list, since effectively none of item attributes can change
            for attr_mutation_request in attr_mutation_requests.iter_mut() {
                self.change_mutation_attr(r_data, attr_mutation_request)?;
            }
            return Ok(Vec::new());
        };
        // Pregenerated attributes ID is valid for the source the cache was generated with, and this
        // method is supposed to be called with the same source
        let unmutated_attrs = r_data
            .get_muta_pregen()
            .get_merged_attrs(mutation_cache.pregen_attrs_rid);
        // Process mutation requests, recording attributes whose values were changed for the item;
        // presized, since growing it costs more than unused capacity
        let mut changed_attr_rids = Vec::with_capacity(attr_mutation_requests.len());
        for attr_mutation_request in attr_mutation_requests.iter_mut() {
            let new_roll = attr_mutation_request.roll;
            // Update user-defined data, putting replaced roll into the request
            attr_mutation_request.roll = match new_roll {
                Some(attr_roll) => item_mutation
                    .attr_rolls
                    .insert(attr_mutation_request.attr_aid, attr_roll),
                None => item_mutation.attr_rolls.remove(&attr_mutation_request.attr_aid),
            };
            // No unmutated value now means there couldn't be any mutated value with any mutation
            // earlier as well, thus attribute value cannot change. We already updated user data,
            // so just go to next attribute
            let Some(attr_rid) = r_data.get_attr_rid_by_aid(&attr_mutation_request.attr_aid) else {
                continue;
            };
            let Some(&unmutated_value) = unmutated_attrs.get(&attr_rid) else {
                continue;
            };
            let Some(mutation_range) = mutation_cache.mutator.attr_mods.get(&attr_rid) else {
                // No mutation range now means there couldn't be any mutated value earlier as well,
                // regardless of user-defined roll data, thus attribute value cannot change. We
                // already updated user data, so just go to next attribute
                continue;
            };
            let new_value = mutate_attr_value(unmutated_value, mutation_range, new_roll);
            // Since unmutated value of the attribute is available by now, we can safely assume that
            // merged attributes have some value too (those are supposed to be built using the same
            // logic as unmutated value)
            let old_value = mutation_cache.flex_data.attrs.insert(attr_rid, new_value).unwrap();
            if old_value != new_value {
                changed_attr_rids.push(attr_rid);
            }
        }
        Ok(changed_attr_rids)
    }
    /// Sets mutator, regenerating mutated item data from scratch.
    pub(in crate::ud::item) fn set_mutator_type_aid_replace(
        &mut self,
        mutator_type_aid: AItemId,
        r_data: &RData,
    ) -> Result<(), ItemMutatedError> {
        let Some(item_mutation) = &mut self.mutation else {
            return Err(ItemMutatedError);
        };
        item_mutation.mutator_type_aid = mutator_type_aid;
        self.update_r_data(r_data);
        Ok(())
    }
    /// Attempt to set mutator, with the condition that rendered item type does not change. Returns
    /// vector with changed attribute values if mutator was updated.
    pub(in crate::ud::item) fn try_set_mutator_type_aid_inplace(
        &mut self,
        mutator_type_aid: AItemId,
        r_data: &RData,
    ) -> Result<Vec<RAttrId>, MutatorInplaceError> {
        let item_mutation = self.mutation.as_mut().ok_or(ItemMutatedError)?;
        let Some(mutation_cache) = &mut item_mutation.cache else {
            // Mutation is not effective, and stays so if new mutator cannot mutate base item, in
            // which case only mutator ID changes
            let base_type_aid = self.base.get_type_aid();
            return match r_data.get_muta_pregen().get_entry(base_type_aid, mutator_type_aid) {
                Some(..) => Err(MutatorInplaceError::MutatedTypeChange),
                None => {
                    item_mutation.mutator_type_aid = mutator_type_aid;
                    Ok(Vec::new())
                }
            };
        };
        let mutator = r_data
            .get_mutator_by_aid(&mutator_type_aid)
            .ok_or(MutatorInplaceError::MutatedTypeChange)?;
        let muta_pregen = r_data.get_muta_pregen();
        // With mutation cache set, item base stores mutated item
        let mutated_r_item = self.base.base_get_r_item().unwrap();
        if muta_pregen
            .get_entry(mutation_cache.base_type_aid, mutator_type_aid)
            .map(|v| v.mutated_item.base.aid)
            != Some(mutated_r_item.base.aid)
        {
            return Err(MutatorInplaceError::MutatedTypeChange);
        }
        // Unmutated attributes depend only on base and mutated items, which do not change, so
        // pregenerated attributes ID stays the same as well
        let unmutated_attrs = muta_pregen.get_merged_attrs(mutation_cache.pregen_attrs_rid);
        // Mutators which share mutated item normally mutate the same attributes
        let mut changed_attr_rids = Vec::with_capacity(mutator.attr_mods.len());
        // Attributes not mutated by new mutator get their unmutated values back
        for &attr_rid in mutation_cache.mutator.attr_mods.keys() {
            if mutator.attr_mods.contains_key(&attr_rid) {
                continue;
            }
            let Some(&unmutated_value) = unmutated_attrs.get(&attr_rid) else {
                continue;
            };
            let old_value = mutation_cache
                .flex_data
                .attrs
                .insert(attr_rid, unmutated_value)
                .unwrap();
            if old_value != unmutated_value {
                changed_attr_rids.push(attr_rid);
            }
        }
        for (&attr_rid, attr_mutation_range) in mutator.attr_mods.iter() {
            let Some(&unmutated_value) = unmutated_attrs.get(&attr_rid) else {
                continue;
            };
            let attr_aid = r_data.get_attr_by_rid(attr_rid).aid;
            let mutated_value = mutate_attr_value(
                unmutated_value,
                attr_mutation_range,
                item_mutation.attr_rolls.get(&attr_aid).copied(),
            );
            let old_value = mutation_cache.flex_data.attrs.insert(attr_rid, mutated_value).unwrap();
            if old_value != mutated_value {
                changed_attr_rids.push(attr_rid);
            }
        }
        item_mutation.mutator_type_aid = mutator_type_aid;
        mutation_cache.mutator = mutator.clone();
        Ok(changed_attr_rids)
    }
    /// Removes mutation, returning it.
    pub(in crate::ud::item) fn unmutate(&mut self, r_data: &RData) -> Result<ItemMutationData, ItemMutatedError> {
        let Some(item_mutation_data) = self.mutation.take() else {
            return Err(ItemMutatedError);
        };
        // If cache is there, mutation is effective - item base has mutated item, and base type ID
        // is stored on cache. No cache - mutation was not effective, and base item was used
        // already, no changes needed
        if let Some(cache) = &item_mutation_data.cache {
            self.base.set_type_aid(cache.base_type_aid, r_data);
        }
        Ok(item_mutation_data)
    }
    /// Puts back mutation taken by unmutating, without deriving anything again.
    pub(in crate::ud::item) fn restore_mutation(&mut self, item_mutation_data: ItemMutationData, r_data: &RData) {
        // Mutation cache is valid as long as data source stays the same, only the mutated item is
        // not stored on it
        if let Some(cache) = &item_mutation_data.cache {
            let pregen_entry = r_data
                .get_muta_pregen()
                .get_entry(cache.base_type_aid, item_mutation_data.mutator_type_aid)
                .unwrap();
            self.base.base_set_r_item(pregen_entry.mutated_item.clone());
        }
        self.mutation = Some(item_mutation_data);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Running effect-specific methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemBaseMutable {
    pub(in crate::ud::item) fn get_reffs(&self) -> Option<&REffs> {
        self.base.get_reffs()
    }
    pub(in crate::ud::item) fn update_reffs(
        &mut self,
        reuse_eupdates: &mut UEffectUpdates,
        r_data: &RData,
        effect_options: RItemEffectOptions,
    ) {
        self.base.update_reffs(reuse_eupdates, r_data, effect_options);
    }
    pub(in crate::ud::item) fn stop_all_reffs(
        &mut self,
        reuse_eupdates: &mut UEffectUpdates,
        r_data: &RData,
        effect_options: RItemEffectOptions,
    ) {
        self.base.stop_all_reffs(reuse_eupdates, r_data, effect_options);
    }
    pub(in crate::ud::item) fn remove_stopped_reffs(&mut self, eupdates: &UEffectUpdates) {
        self.base.remove_stopped_reffs(eupdates);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Auxiliary entities
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct ItemMutationData {
    // User-defined data
    mutator_type_aid: AItemId,
    attr_rolls: RMap<AAttrId, UnitInterval>,
    // Source-dependent data
    cache: Option<ItemMutationDataCache>,
}
impl ItemMutationData {
    fn with_attrs(mutator_type_aid: AItemId, attr_rolls: RMap<AAttrId, UnitInterval>) -> Self {
        Self {
            mutator_type_aid,
            attr_rolls,
            cache: None,
        }
    }
    pub(crate) fn get_mutator_type_aid(&self) -> AItemId {
        self.mutator_type_aid
    }
    pub(crate) fn get_attr_rolls(&self) -> &RMap<AAttrId, UnitInterval> {
        &self.attr_rolls
    }
    pub(crate) fn get_cache(&self) -> Option<&ItemMutationDataCache> {
        self.cache.as_ref()
    }
    pub(crate) fn is_effective(&self) -> bool {
        self.cache.is_some()
    }
}

fn convert_request_to_data(mutation_request: UItemMutationRequest) -> ItemMutationData {
    ItemMutationData::with_attrs(
        mutation_request.mutator_type_aid,
        mutation_request
            .attrs
            .into_iter()
            .filter_map(|attr_mutation| attr_mutation.roll.map(|roll| (attr_mutation.attr_aid, roll)))
            .collect(),
    )
}

#[derive(Clone)]
pub(crate) struct ItemMutationDataCache {
    base_type_aid: AItemId,
    mutator: RcMuta,
    flex_data: RItemFlexData,
    /// ID which is used to locate pregenerated merged attributes (which are not mutated)
    pregen_attrs_rid: RMutasPregenMergedAttrsId,
}
impl ItemMutationDataCache {
    pub(crate) fn get_base_type_aid(&self) -> AItemId {
        self.base_type_aid
    }
    pub(crate) fn get_r_mutator(&self) -> &RMuta {
        &self.mutator
    }
    pub(crate) fn get_pregen_attrs_rid(&self) -> RMutasPregenMergedAttrsId {
        self.pregen_attrs_rid
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Attribute mutations
////////////////////////////////////////////////////////////////////////////////////////////////////
fn apply_attr_mutations(
    item_flex_data: &mut RItemFlexData,
    mutator: &RMuta,
    attr_rolls: &RMap<AAttrId, UnitInterval>,
    r_data: &RData,
) {
    for (&attr_rid, attr_mutation_range) in mutator.attr_mods.iter() {
        let Some(&unmutated_value) = item_flex_data.attrs.get(&attr_rid) else {
            continue;
        };
        let attr_id = r_data.get_attr_by_rid(attr_rid).aid;
        let mutated_val = mutate_attr_value(unmutated_value, attr_mutation_range, attr_rolls.get(&attr_id).copied());
        item_flex_data.attrs.insert(attr_rid, mutated_val);
    }
}

fn mutate_attr_value(unmutated_value: Value, roll_range: &RMutaAttrRange, roll: Option<UnitInterval>) -> Value {
    if let Some(roll) = roll {
        return unmutated_value
            * (roll_range.mult_min_raw + roll.into_value() * (roll_range.mult_max_raw - roll_range.mult_min_raw));
    }
    // When no roll is defined by user, still limit possible values by what roll range is
    if roll_range.mult_min_math >= Value::ONE {
        return unmutated_value * roll_range.mult_min_math;
    }
    if roll_range.mult_max_math <= Value::ONE {
        return unmutated_value * roll_range.mult_max_math;
    }
    unmutated_value
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl UItemBaseMutable {
        pub(in crate::ud::item) fn consistency_check(
            &self,
            u_data: &UData,
            effect_options: RItemEffectOptions,
        ) -> DebugResult {
            let Self { base, mutation } = self;
            base.consistency_check(u_data, effect_options)?;
            if let Some(mutation_data) = mutation {
                mutation_data.consistency_check(u_data)?;
            }
            Ok(())
        }
    }

    impl ItemMutationData {
        fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self {
                cache,
                // Not checked
                mutator_type_aid: _,
                attr_rolls: _,
            } = self;
            if let Some(mutation_cache) = cache {
                mutation_cache.consistency_check(u_data)?;
            }
            Ok(())
        }
    }

    impl ItemMutationDataCache {
        fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self {
                base_type_aid,
                pregen_attrs_rid,
                mutator,
                flex_data,
            } = self;
            if u_data
                .r_data
                .get_muta_pregen()
                .get_entry(*base_type_aid, mutator.id)
                .map(|v| v.merged_attrs_rid)
                != Some(*pregen_attrs_rid)
            {
                return Err(Default::default());
            }
            flex_data.consistency_check(u_data)?;
            Ok(())
        }
    }
}
