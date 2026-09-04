use crate::{
    EffectMode, ItemId, UnitInterval, Value,
    ad::{AAttrId, AEffectId, AItemId},
    err::basic::ItemNotMutatedError,
    rd::{RAttrId, RData, REffectId, RItem, RItemAttrData, RItemBase, RMuta, RMutaAttrRange, RState, RcItem, RcMuta},
    ud::{
        UAttrMutationRequest, UItemMutationRequest,
        item::base::{UEffectUpdates, UItemBase, mutable::err::ItemMutatedError},
    },
    util::{RMap, RSet},
};

// Item mutable base stores all the data every mutable item should have.
//
// Mutated item can have 3 states:
// - Non-mutated - mutation is not set, only base item info is used
// - Mutated, mutation is loaded - source had all the needed mutation data, which was processed and
//   stored on cache. In this case, item base stores mutated item type, and base aitem ID is stored
//   on mutation cache;
// - Mutated, mutation not loaded - item base stores base item type, mutation stores mutator ID and
//   attribute mutations, and mutation cache isn't set.
//
// Mutated items expose RItemBase of mutated item, and RItemAttrData which contains "hybrid" data:
// - Attribute values - merged attributes (base item attrs + mutated item attrs) with all the
//   mutations applied;
// - Everything else (per-effect attr-derived data, on-item attr-derived data) is built using merged
//   attributes, but not modified by mutations (it also means mutating attributes does not change
//   this part). Those properties are stored here for performance reasons, and are supposed to be
//   "immutable". Them being unchangeable by mutations is intentional. If there was need to use
//   mutated/modified value, those properties could be recalculated by request using attribute
//   values fetched from calc.
#[derive(Clone)]
pub(in crate::ud::item) struct UItemBaseMutable {
    pub(super) base: UItemBase,
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
        let Some(mutated_r_item) = mutator.item_map.get(&type_aid).and_then(|v| r_data.get_item_by_aid(v)) else {
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
        let merged_attrs = get_combined_attr_values(r_data.get_item_by_aid(&type_aid), mutated_r_item);
        let mut merged_attr_data = RItemAttrData::from_attrs(merged_attrs, &mutated_r_item.base, r_data);
        apply_attr_mutations(&mut merged_attr_data, mutator, &item_mutation_data.attr_rolls, r_data);
        let regular_base = UItemBase::base_with_r_item(item_id, mutated_r_item.clone(), state);
        item_mutation_data.cache = Some(ItemMutationDataCache {
            base_type_aid: type_aid,
            mutator: mutator.clone(),
            merged_attr_data,
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
    pub(in crate::ud::item) fn get_r_item_attr_data(&self) -> Option<&RItemAttrData> {
        let Some(item_mutation) = &self.mutation else {
            return self.base.get_r_item_attr_data();
        };
        match &item_mutation.cache {
            Some(cache) => Some(&cache.merged_attr_data),
            None => self.base.get_r_item_attr_data(),
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
    pub(in crate::ud::item) fn get_type_aid(&self) -> AItemId {
        self.base.get_type_aid()
    }
    pub(in crate::ud::item) fn set_type_aid(&mut self, type_aid: AItemId, r_data: &RData) {
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

    pub(in crate::ud::item) fn get_state(&self) -> RState {
        self.base.get_state()
    }
    pub(in crate::ud::item) fn set_state(&mut self, state: RState) {
        self.base.set_state(state)
    }
    pub(in crate::ud::item) fn get_effect_mode_by_rid(&self, effect_rid: &REffectId) -> EffectMode {
        self.base.get_effect_mode_by_rid(effect_rid)
    }
    pub(in crate::ud::item) fn iter_effect_mode_overrides(
        &self,
    ) -> impl ExactSizeIterator<Item = (AEffectId, EffectMode)> {
        self.base.iter_effect_mode_overrides()
    }
    pub(in crate::ud::item) fn set_effect_mode(
        &mut self,
        effect_aid: AEffectId,
        effect_mode: EffectMode,
        r_data: &RData,
    ) {
        self.base.set_effect_mode(effect_aid, effect_mode, r_data)
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        r_data: &RData,
    ) {
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
        let Some(mutated_r_item) = mutator
            .item_map
            .get(&base_type_aid)
            .and_then(|v| r_data.get_item_by_aid(v))
        else {
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
        let merged_attrs = get_combined_attr_values(r_data.get_item_by_aid(&base_type_aid), mutated_r_item);
        let mut merged_attr_data = RItemAttrData::from_attrs(merged_attrs, &mutated_r_item.base, r_data);
        apply_attr_mutations(&mut merged_attr_data, mutator, &item_mutation.attr_rolls, r_data);
        // Everything needed is at hand, update item
        self.base.base_set_r_item(mutated_r_item.clone());
        item_mutation.cache = Some(ItemMutationDataCache {
            base_type_aid,
            mutator: mutator.clone(),
            merged_attr_data,
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
        let Some(mutated_r_item) = mutator
            .item_map
            .get(&base_type_aid)
            .and_then(|v| r_data.get_item_by_aid(v))
        else {
            // No mutated aitem ID or no mutated item itself - nothing changes, except for
            // user-defined mutations getting stored
            self.mutation = Some(item_mutation_data);
            return Ok(());
        };
        // Since we have all the data now, apply mutation properly
        let merged_attrs = get_combined_attr_values(self.base.base_get_r_item(), mutated_r_item);
        let mut merged_attr_data = RItemAttrData::from_attrs(merged_attrs, &mutated_r_item.base, r_data);
        apply_attr_mutations(&mut merged_attr_data, mutator, &item_mutation_data.attr_rolls, r_data);
        self.base.base_set_r_item(mutated_r_item.clone());
        item_mutation_data.cache = Some(ItemMutationDataCache {
            base_type_aid,
            mutator: mutator.clone(),
            merged_attr_data,
        });
        self.mutation = Some(item_mutation_data);
        Ok(())
    }
    pub(in crate::ud::item) fn change_mutation_attrs(
        &mut self,
        r_data: &RData,
        attr_mutation_requests: Vec<UAttrMutationRequest>,
    ) -> Result<Vec<RAttrId>, ItemMutatedError> {
        let Some(item_mutation) = &mut self.mutation else {
            return Err(ItemMutatedError);
        };
        let Some(mutation_cache) = &mut item_mutation.cache else {
            // If there is no cache - mutations are not effective. In this case we update user data
            // and return empty list, since effectively none of item attributes can change
            for attr_mutation_request in attr_mutation_requests {
                match attr_mutation_request.roll {
                    Some(roll_val) => {
                        item_mutation
                            .attr_rolls
                            .insert(attr_mutation_request.attr_aid, roll_val);
                    }
                    None => {
                        item_mutation.attr_rolls.remove(&attr_mutation_request.attr_aid);
                    }
                }
            }
            return Ok(Vec::new());
        };
        // All the methods which set cache guarantee that all the following entities are available
        // for the source the cache was generated with, and this method is supposed to be called
        // with the same source
        let mutated_type_aid = mutation_cache
            .mutator
            .item_map
            .get(&mutation_cache.base_type_aid)
            .unwrap();
        let mutated_r_item = r_data.get_item_by_aid(mutated_type_aid).unwrap();
        // Process mutation requests, recording attributes whose values were changed for the item
        let mut base_r_item_cache = None;
        let mut changed_attr_rids = Vec::new();
        for attr_mutation_request in attr_mutation_requests {
            let new_rid_value = match attr_mutation_request.roll {
                // Mutation change request
                Some(attr_roll) => {
                    // Update user-defined data
                    item_mutation
                        .attr_rolls
                        .insert(attr_mutation_request.attr_aid, attr_roll);
                    // Process source-dependent data and return new value
                    let Some(unmutated_value) = get_combined_attr_value(
                        r_data,
                        &mutation_cache.base_type_aid,
                        &mut base_r_item_cache,
                        mutated_r_item,
                        &attr_mutation_request.attr_aid,
                    ) else {
                        // No unmutated value now means there couldn't be any mutated value with any
                        // mutation earlier as well, thus attribute value cannot change. We already
                        // updated user data, so just go to next attribute
                        continue;
                    };
                    let Some(mutation_range) = mutation_cache.mutator.attr_mods.get(&unmutated_value.rid) else {
                        // No mutation range now means there couldn't be any mutated value earlier
                        // as well, regardless of user-defined roll data, thus attribute value
                        // cannot change. We already updated user data, so just go to next attribute
                        continue;
                    };
                    AttrRidVal {
                        rid: unmutated_value.rid,
                        value: mutate_attr_value(unmutated_value.value, mutation_range, attr_roll),
                    }
                }
                // Mutation removal request
                None => {
                    // Update user-defined data
                    item_mutation.attr_rolls.remove(&attr_mutation_request.attr_aid);
                    // Update source-dependent data
                    let Some(unmutated_value) = get_combined_attr_value(
                        r_data,
                        &mutation_cache.base_type_aid,
                        &mut base_r_item_cache,
                        mutated_r_item,
                        &attr_mutation_request.attr_aid,
                    ) else {
                        // No unmutated value - can't do any comparisons
                        continue;
                    };
                    AttrRidVal {
                        rid: unmutated_value.rid,
                        // Limit possible values by roll range, if it is available
                        value: match mutation_cache.mutator.attr_mods.get(&unmutated_value.rid) {
                            Some(mutation_range) => limit_attr_value(unmutated_value.value, mutation_range),
                            None => unmutated_value.value,
                        },
                    }
                }
            };
            // Since unmutated value of the attribute is available by now, we can safely assume that
            // merged attributes have some value too (those are supposed to be built using the same
            // logic as unmutated value)
            let old_value = mutation_cache
                .merged_attr_data
                .attrs
                .insert(new_rid_value.rid, new_rid_value.value)
                .unwrap();
            if old_value != new_rid_value.value {
                changed_attr_rids.push(new_rid_value.rid);
            }
        }
        Ok(changed_attr_rids)
    }
    pub(in crate::ud::item) fn set_mutator_type_aid(
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
    pub(in crate::ud::item) fn unmutate(&mut self, r_data: &RData) -> Result<(), ItemMutatedError> {
        let Some(item_mutation) = &mut self.mutation else {
            return Err(ItemMutatedError);
        };
        match &item_mutation.cache {
            // If cache is there, mutation is effective - item base has mutated item, and base type
            // ID is stored on cache
            Some(cache) => {
                let type_aid = cache.base_type_aid;
                self.base.set_type_aid(type_aid, r_data);
                self.mutation = None;
            }
            // No cache - mutation was not effective, and base item was used already, no changes
            // needed, just unassign mutation in this case
            None => {
                self.mutation = None;
            }
        };
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Running effect-specific methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemBaseMutable {
    pub(in crate::ud::item) fn get_reffs(&self) -> Option<&RSet<REffectId>> {
        self.base.get_reffs()
    }
    pub(in crate::ud::item) fn update_reffs(
        &mut self,
        reuse_eupdates: &mut UEffectUpdates,
        r_data: &RData,
        require_disabled_defeff: bool,
        force_active_nondefeff: bool,
    ) {
        self.base
            .update_reffs(reuse_eupdates, r_data, require_disabled_defeff, force_active_nondefeff);
    }
    pub(in crate::ud::item) fn stop_all_reffs(
        &mut self,
        reuse_eupdates: &mut UEffectUpdates,
        r_data: &RData,
        require_disabled_defeff: bool,
        force_active_nondefeff: bool,
    ) {
        self.base
            .stop_all_reffs(reuse_eupdates, r_data, require_disabled_defeff, force_active_nondefeff);
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
    pub(super) merged_attr_data: RItemAttrData,
}
impl ItemMutationDataCache {
    pub(crate) fn get_base_type_aid(&self) -> AItemId {
        self.base_type_aid
    }
    pub(crate) fn get_r_mutator(&self) -> &RMuta {
        &self.mutator
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Attribute mutations
////////////////////////////////////////////////////////////////////////////////////////////////////
fn apply_attr_mutations(
    item_attr_data: &mut RItemAttrData,
    mutator: &RMuta,
    attr_rolls: &RMap<AAttrId, UnitInterval>,
    r_data: &RData,
) {
    for (&attr_rid, attr_mutation_range) in mutator.attr_mods.iter() {
        let Some(&unmutated_value) = item_attr_data.attrs.get(&attr_rid) else {
            continue;
        };
        let attr_id = r_data.get_attr_by_rid(attr_rid).aid;
        match attr_rolls.get(&attr_id) {
            Some(attr_roll) => {
                let mutated_val = mutate_attr_value(unmutated_value, attr_mutation_range, *attr_roll);
                item_attr_data.attrs.insert(attr_rid, mutated_val);
            }
            // When no roll is defined by user, still limit possible values by what roll range is
            None => {
                let mutated_val = limit_attr_value(unmutated_value, attr_mutation_range);
                item_attr_data.attrs.insert(attr_rid, mutated_val);
            }
        }
    }
}

fn mutate_attr_value(unmutated_value: Value, roll_range: &RMutaAttrRange, roll: UnitInterval) -> Value {
    unmutated_value
        * (roll_range.mult_min_raw + roll.into_value() * (roll_range.mult_max_raw - roll_range.mult_min_raw))
}

fn limit_attr_value(unmutated_value: Value, roll_range: &RMutaAttrRange) -> Value {
    if roll_range.mult_min_math >= Value::ONE {
        return unmutated_value * roll_range.mult_min_math;
    }
    if roll_range.mult_max_math <= Value::ONE {
        return unmutated_value * roll_range.mult_max_math;
    }
    unmutated_value
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
struct AttrRidVal {
    rid: RAttrId,
    value: Value,
}

fn get_combined_attr_value<'a>(
    r_data: &'a RData,
    base_type_aid: &AItemId,
    base_r_item_cache: &mut Option<Option<&'a RcItem>>,
    mutated_r_item: &RItem,
    attr_id: &AAttrId,
) -> Option<AttrRidVal> {
    let attr_rid = r_data.get_attr_rid_by_aid(attr_id)?;
    let value = match mutated_r_item.attr_data.attrs.get(&attr_rid) {
        Some(&unmutated_value) => Some(unmutated_value),
        None => match base_r_item_cache {
            Some(opt_base_r_item) => {
                opt_base_r_item.and_then(|base_r_item| base_r_item.attr_data.attrs.get(&attr_rid).copied())
            }
            None => {
                let opt_base_r_item = r_data.get_item_by_aid(base_type_aid);
                base_r_item_cache.replace(opt_base_r_item);
                opt_base_r_item.and_then(|base_r_item| base_r_item.attr_data.attrs.get(&attr_rid).copied())
            }
        },
    }?;
    Some(AttrRidVal { rid: attr_rid, value })
}

pub(crate) fn get_combined_attr_values(base_r_item: Option<&RcItem>, mutated_r_item: &RItem) -> RMap<RAttrId, Value> {
    match base_r_item {
        Some(base_r_item) => {
            let mut attrs = base_r_item.attr_data.attrs.clone();
            // Mutated item attributes have priority in case of collisions
            for (&attr_rid, &attr_val) in mutated_r_item.attr_data.attrs.iter() {
                attrs.insert(attr_rid, attr_val);
            }
            attrs
        }
        None => mutated_r_item.attr_data.attrs.clone(),
    }
}
