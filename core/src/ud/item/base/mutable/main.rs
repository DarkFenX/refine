use crate::{
    ad::{
        AAttrId, AAttrVal, AEffectId, AEveItemListId, AItemCatId, AItemGrpId, AItemId, AItemListId, AMutaAttrRange,
        ASkillLevel, AState,
    },
    def::{ItemId, OF},
    err::basic::ItemNotMutatedError,
    misc::{AttrMutationRequest, EffectMode, ItemMutationRequest},
    rd::{RAttrKey, REffectKey, RItem, RItemAXt, RItemEffectData, RItemListKey, RMuta, RcItem, RcMuta},
    src::Src,
    ud::{
        err::ItemMutatedError,
        item::base::{UEffectUpdates, UItemBase},
    },
    util::{RMap, RSet, UnitInterval},
};

// Item mutable base stores all the data every mutable item should have.
//
// Mutated item can have 3 states:
// - Non-mutated - mutation is not set, only base item info is used
// - Mutated, mutation is loaded - source had all the needed mutation data, which was processed and
// stored on cache. In this case, item base stores mutated item type, and base aitem ID is stored on
// mutation cache;
// - Mutated, mutation not loaded - item base stores base item type, mutation stores mutator ID and
// attribute mutations, and mutation cache isn't set.
#[derive(Clone)]
pub(in crate::ud::item) struct UItemBaseMutable {
    pub(super) base: UItemBase,
    mutation: Option<ItemMutationData>,
}
impl UItemBaseMutable {
    pub(in crate::ud::item) fn new(
        item_id: ItemId,
        type_id: AItemId,
        state: AState,
        mutation_request: Option<ItemMutationRequest>,
        src: &Src,
    ) -> Self {
        let mutation_request = match mutation_request {
            Some(mutation_request) => mutation_request,
            // No mutation - regular non-mutated item setup
            None => {
                return Self {
                    base: UItemBase::new(item_id, type_id, state, src),
                    mutation: None,
                };
            }
        };
        let mutator_id = mutation_request.mutator_id;
        let mut item_mutation_data = convert_request_to_data(mutation_request);
        let mutator = match src.get_mutator(&mutator_id) {
            Some(mutator) => mutator,
            // No mutator - base item with ineffective user-defined mutations
            None => {
                return Self {
                    base: UItemBase::new(item_id, type_id, state, src),
                    mutation: Some(item_mutation_data),
                };
            }
        };
        // No mutated item ID in mapping or no mutated item itself
        let mutated_r_item = match mutator.item_map.get(&type_id).and_then(|v| src.get_item(v)) {
            Some(mutated_r_item) => mutated_r_item,
            None => {
                return match src.get_item(&type_id) {
                    // If base item is available, return base item, but with ineffective
                    // user-defined mutations
                    Some(base_r_item) => Self {
                        base: UItemBase::base_new_with_r_item(item_id, base_r_item.clone(), state),
                        mutation: Some(item_mutation_data),
                    },
                    // No base item - unloaded item with ineffective user-defined mutations
                    None => Self {
                        base: UItemBase::base_new_with_type_id_not_loaded(item_id, type_id, state),
                        mutation: Some(item_mutation_data),
                    },
                };
            }
        };
        // Make proper mutated item once we have all the data
        let mut merged_attrs = get_combined_attr_values(src.get_item(&type_id), mutated_r_item);
        let merged_effdatas = merge_effect_datas(mutated_r_item, &merged_attrs, src);
        let item_axt = make_axt(mutated_r_item, &merged_attrs, merged_effdatas.as_ref(), src);
        apply_attr_mutations(&mut merged_attrs, mutator, &item_mutation_data.attr_rolls, src);
        let regular_base = UItemBase::base_new_with_r_item(item_id, mutated_r_item.clone(), state);
        item_mutation_data.cache = Some(ItemMutationDataCache {
            base_type_id: type_id,
            mutator: mutator.clone(),
            merged_attrs,
            merged_effdatas,
            axt: item_axt,
        });
        Self {
            base: regular_base,
            mutation: Some(item_mutation_data),
        }
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Basic data access methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub(in crate::ud::item) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(in crate::ud::item) fn get_type_id(&self) -> AItemId {
        self.base.get_type_id()
    }
    pub(in crate::ud::item) fn set_type_id(&mut self, type_id: AItemId, src: &Src) {
        // Since this method is supposed to update base item ID for mutated items, location of ID
        // depends on item configuration
        match &mut self.mutation {
            Some(mutation_data) => match &mut mutation_data.cache {
                Some(mutation_cache) => {
                    mutation_cache.base_type_id = type_id;
                }
                None => self.base.base_set_type_id_primitive(type_id),
            },
            None => self.base.base_set_type_id_primitive(type_id),
        }
        // Even if mutation is not effective with old base type ID, it might become effective with
        // the new one, so - update the data the mutated way regardless of presence of the mutation
        // cache
        self.update_r_data(src);
    }
    pub(in crate::ud::item) fn get_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_group_id()
    }
    pub(in crate::ud::item) fn get_category_id(&self) -> Option<AItemCatId> {
        self.base.get_category_id()
    }
    pub(in crate::ud::item) fn get_attrs(&self) -> Option<&RMap<RAttrKey, AAttrVal>> {
        let item_mutation = match &self.mutation {
            Some(item_mutation) => item_mutation,
            None => return self.base.get_attrs(),
        };
        match &item_mutation.cache {
            Some(cache) => Some(&cache.merged_attrs),
            None => self.base.get_attrs(),
        }
    }
    pub(in crate::ud::item) fn get_effect_datas(&self) -> Option<&RMap<REffectKey, RItemEffectData>> {
        // Merged effect data is set only if mutation is valid, and if it contained any differences
        // to mutated item effect data
        if let Some(item_mutation) = &self.mutation
            && let Some(mutation_cache) = &item_mutation.cache
            && let Some(merged_effect_datas) = &mutation_cache.merged_effdatas
        {
            return Some(merged_effect_datas);
        }
        self.base.get_effect_datas()
    }
    pub(in crate::ud::item) fn get_defeff_key(&self) -> Option<Option<REffectKey>> {
        self.base.get_defeff_key()
    }
    pub(in crate::ud::item) fn get_skill_reqs(&self) -> Option<&RMap<AItemId, ASkillLevel>> {
        self.base.get_skill_reqs()
    }
    pub(in crate::ud::item) fn get_proj_buff_item_lists(&self) -> Option<&Vec<RItemListKey>> {
        self.base.get_proj_buff_item_lists()
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Extra data access methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub(in crate::ud::item) fn get_axt(&self) -> Option<&RItemAXt> {
        if let Some(item_mutation) = &self.mutation
            && let Some(mutation_cache) = &item_mutation.cache
        {
            return Some(&mutation_cache.axt);
        }
        self.base.get_axt()
    }
    pub(in crate::ud::item) fn get_max_state(&self) -> Option<AState> {
        self.base.get_max_state()
    }
    pub(in crate::ud::item) fn get_val_fitted_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_val_fitted_group_id()
    }
    pub(in crate::ud::item) fn get_val_online_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_val_online_group_id()
    }
    pub(in crate::ud::item) fn get_val_active_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_val_active_group_id()
    }
    pub(in crate::ud::item) fn takes_turret_hardpoint(&self) -> bool {
        self.base.takes_turret_hardpoint()
    }
    pub(in crate::ud::item) fn takes_launcher_hardpoint(&self) -> bool {
        self.base.takes_launcher_hardpoint()
    }
    pub(in crate::ud::item) fn is_ice_harvester(&self) -> bool {
        self.base.is_ice_harvester()
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////
    // Misc methods
    ////////////////////////////////////////////////////////////////////////////////////////////////////
    pub(in crate::ud::item) fn get_reffs(&self) -> Option<&RSet<REffectKey>> {
        self.base.get_reffs()
    }
    pub(in crate::ud::item) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.update_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.stop_all_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn get_state(&self) -> AState {
        self.base.get_state()
    }
    pub(in crate::ud::item) fn set_state(&mut self, state: AState) {
        self.base.set_state(state)
    }
    pub(in crate::ud::item) fn get_effect_key_mode(&self, effect_key: &REffectKey) -> EffectMode {
        self.base.get_effect_key_mode(effect_key)
    }
    pub(in crate::ud::item) fn set_effect_mode(&mut self, effect_id: AEffectId, effect_mode: EffectMode, src: &Src) {
        self.base.set_effect_mode(effect_id, effect_mode, src)
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        src: &Src,
    ) {
        self.base.set_effect_modes(modes, src)
    }
    pub(in crate::ud::item) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::ud::item) fn src_changed(&mut self, src: &Src) {
        self.base.base_update_effect_modes(src);
        self.update_r_data(src);
    }
    fn update_r_data(&mut self, src: &Src) {
        let item_mutation = match &mut self.mutation {
            Some(item_mutation) => item_mutation,
            // No mutation - just update base item
            None => {
                self.base.base_update_r_data(src);
                return;
            }
        };
        let base_type_id = match &item_mutation.cache {
            Some(cache) => cache.base_type_id,
            None => self.base.get_type_id(),
        };
        let mutator = match src.get_mutator(&item_mutation.mutator_id) {
            Some(mutator) => mutator,
            // No mutator - invalidate mutated cache and use non-mutated item
            None => match src.get_item(&base_type_id) {
                Some(base_r_item) => {
                    self.base.base_set_r_item(base_r_item.clone());
                    item_mutation.cache = None;
                    return;
                }
                None => {
                    self.base.base_set_type_id_not_loaded(base_type_id);
                    item_mutation.cache = None;
                    return;
                }
            },
        };
        let mutated_r_item = match mutator.item_map.get(&base_type_id).and_then(|v| src.get_item(v)) {
            Some(mutated_r_item) => mutated_r_item,
            // No mutated aitem ID or no item itself - invalidate mutated cache and use non-mutated
            // item
            None => match src.get_item(&base_type_id) {
                Some(base_r_item) => {
                    self.base.base_set_r_item(base_r_item.clone());
                    item_mutation.cache = None;
                    return;
                }
                None => {
                    self.base.base_set_type_id_not_loaded(base_type_id);
                    item_mutation.cache = None;
                    return;
                }
            },
        };
        // Compose attribute cache
        let mut merged_attrs = get_combined_attr_values(src.get_item(&base_type_id), mutated_r_item);
        let merged_effdatas = merge_effect_datas(mutated_r_item, &merged_attrs, src);
        let item_axt = make_axt(mutated_r_item, &merged_attrs, merged_effdatas.as_ref(), src);
        apply_attr_mutations(&mut merged_attrs, mutator, &item_mutation.attr_rolls, src);
        // Everything needed is at hand, update item
        self.base.base_set_r_item(mutated_r_item.clone());
        item_mutation.cache = Some(ItemMutationDataCache {
            base_type_id,
            mutator: mutator.clone(),
            merged_attrs,
            merged_effdatas,
            axt: item_axt,
        })
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Mutation-specific methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub(in crate::ud::item) fn get_mutation_data(&self) -> Option<&ItemMutationData> {
        self.mutation.as_ref()
    }
    pub(in crate::ud::item) fn mutate(
        &mut self,
        mutation_request: ItemMutationRequest,
        src: &Src,
    ) -> Result<(), ItemNotMutatedError> {
        if self.get_mutation_data().is_some() {
            return Err(ItemNotMutatedError {
                item_id: self.get_item_id(),
            });
        };
        // Since item is not mutated, base aitem ID is always on non-mutated item base
        let base_type_id = self.base.get_type_id();
        let mutator_id = mutation_request.mutator_id;
        let mut item_mutation_data = convert_request_to_data(mutation_request);
        let mutator = match src.get_mutator(&mutator_id) {
            Some(mutator) => mutator,
            // No mutator - nothing changes, except for user-defined mutations getting stored
            None => {
                self.mutation = Some(item_mutation_data);
                return Ok(());
            }
        };
        let mutated_r_item = match mutator.item_map.get(&base_type_id).and_then(|v| src.get_item(v)) {
            Some(mutated_r_item) => mutated_r_item,
            // No mutated aitem ID or no mutated item itself - nothing changes, except for
            // user-defined mutations getting stored
            None => {
                self.mutation = Some(item_mutation_data);
                return Ok(());
            }
        };
        // Since we have all the data now, apply mutation properly
        let mut merged_attrs = get_combined_attr_values(self.base.base_get_r_item(), mutated_r_item);
        let merged_effdatas = merge_effect_datas(mutated_r_item, &merged_attrs, src);
        let item_axt = make_axt(mutated_r_item, &merged_attrs, merged_effdatas.as_ref(), src);
        apply_attr_mutations(&mut merged_attrs, mutator, &item_mutation_data.attr_rolls, src);
        self.base.base_set_r_item(mutated_r_item.clone());
        item_mutation_data.cache = Some(ItemMutationDataCache {
            base_type_id,
            mutator: mutator.clone(),
            merged_attrs,
            merged_effdatas,
            axt: item_axt,
        });
        self.mutation = Some(item_mutation_data);
        Ok(())
    }
    pub(in crate::ud::item) fn change_mutation_attrs(
        &mut self,
        src: &Src,
        attr_mutation_requests: Vec<AttrMutationRequest>,
    ) -> Result<Vec<RAttrKey>, ItemMutatedError> {
        let item_mutation = match &mut self.mutation {
            Some(item_mutation) => item_mutation,
            None => {
                return Err(ItemMutatedError {
                    item_id: self.get_item_id(),
                });
            }
        };
        let mutation_cache = match &mut item_mutation.cache {
            Some(cache) => cache,
            // If there is no cache - mutations are not effective. In this case we update user data
            // and return empty list, since effectively none of item attributes can change
            None => {
                for attr_mutation_request in attr_mutation_requests {
                    match attr_mutation_request.value {
                        Some(roll_val) => {
                            item_mutation.attr_rolls.insert(attr_mutation_request.attr_id, roll_val);
                        }
                        None => {
                            item_mutation.attr_rolls.remove(&attr_mutation_request.attr_id);
                        }
                    }
                }
                return Ok(Vec::new());
            }
        };
        // All the methods which set cache guarantee that all the following entities are available
        // for the source the cache was generated with, and this method is supposed to be called
        // with the same source
        let mutated_type_id = mutation_cache
            .mutator
            .item_map
            .get(&mutation_cache.base_type_id)
            .unwrap();
        let mutated_r_item = src.get_item(mutated_type_id).unwrap();
        // Process mutation requests, recording attributes whose values were changed for the item
        let mut base_r_item_cache = None;
        let mut changed_attr_keys = Vec::new();
        for attr_mutation_request in attr_mutation_requests {
            let new_key_value = match attr_mutation_request.value {
                // Mutation change request
                Some(attr_roll) => {
                    // Update user-defined data
                    item_mutation
                        .attr_rolls
                        .insert(attr_mutation_request.attr_id, attr_roll);
                    // Process source-dependent data and return new value
                    let unmutated_value = match get_combined_attr_value(
                        src,
                        &mutation_cache.base_type_id,
                        &mut base_r_item_cache,
                        mutated_r_item,
                        &attr_mutation_request.attr_id,
                    ) {
                        Some(unmutated_value) => unmutated_value,
                        // No unmutated value now means there couldn't be any mutated value with any
                        // mutation earlier as well, thus attribute value cannot change. We already
                        // updated user data, so just go to next attribute
                        None => continue,
                    };
                    let mutation_range = match mutation_cache.mutator.attr_mods.get(&unmutated_value.key) {
                        Some(mutation_range) => mutation_range,
                        // No mutation range now means there couldn't be any mutated value
                        // earlier as well, regardless of user-defined roll data, thus attribute
                        // value cannot change. We already updated user data, so just go to next
                        // attribute
                        None => continue,
                    };
                    AttrKeyVal {
                        key: unmutated_value.key,
                        value: mutate_attr_value(unmutated_value.value, mutation_range, attr_roll),
                    }
                }
                // Mutation removal request
                None => {
                    // Update user-defined data
                    item_mutation.attr_rolls.remove(&attr_mutation_request.attr_id);
                    // Update source-dependent data
                    let unmutated_value = match get_combined_attr_value(
                        src,
                        &mutation_cache.base_type_id,
                        &mut base_r_item_cache,
                        mutated_r_item,
                        &attr_mutation_request.attr_id,
                    ) {
                        Some(unmutated_value) => unmutated_value,
                        // No unmutated value - can't do any comparisons
                        None => continue,
                    };
                    AttrKeyVal {
                        key: unmutated_value.key,
                        // Limit possible values by roll range, if it is available
                        value: match mutation_cache.mutator.attr_mods.get(&unmutated_value.key) {
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
                .merged_attrs
                .insert(new_key_value.key, new_key_value.value)
                .unwrap();
            if old_value != new_key_value.value {
                changed_attr_keys.push(new_key_value.key);
            }
        }
        Ok(changed_attr_keys)
    }
    pub(in crate::ud::item) fn set_mutator_id(
        &mut self,
        mutator_id: AItemId,
        src: &Src,
    ) -> Result<(), ItemMutatedError> {
        let item_mutation = match &mut self.mutation {
            Some(item_mutation) => item_mutation,
            None => {
                return Err(ItemMutatedError {
                    item_id: self.get_item_id(),
                });
            }
        };
        item_mutation.mutator_id = mutator_id;
        self.update_r_data(src);
        Ok(())
    }
    pub(in crate::ud::item) fn unmutate(&mut self, src: &Src) -> Result<(), ItemMutatedError> {
        let item_mutation = match &mut self.mutation {
            Some(item_mutation) => item_mutation,
            None => {
                return Err(ItemMutatedError {
                    item_id: self.get_item_id(),
                });
            }
        };
        match &item_mutation.cache {
            // If cache is there, mutation is effective - item base has mutated item, and base type
            // ID is stored on cache
            Some(cache) => {
                let type_id = cache.base_type_id;
                self.base.set_type_id(type_id, src);
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

#[derive(Clone)]
pub(crate) struct ItemMutationData {
    // User-defined data
    mutator_id: AItemId,
    attr_rolls: RMap<AAttrId, UnitInterval>,
    // Source-dependent data
    cache: Option<ItemMutationDataCache>,
}
impl ItemMutationData {
    fn new_with_attrs(mutator_id: AItemId, attr_rolls: RMap<AAttrId, UnitInterval>) -> Self {
        Self {
            mutator_id,
            attr_rolls,
            cache: None,
        }
    }
    pub(crate) fn get_mutator_id(&self) -> AItemId {
        self.mutator_id
    }
    pub(crate) fn get_attr_rolls(&self) -> &RMap<AAttrId, UnitInterval> {
        &self.attr_rolls
    }
    pub(crate) fn get_cache(&self) -> Option<&ItemMutationDataCache> {
        self.cache.as_ref()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Container for data which is source-dependent
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct ItemMutationDataCache {
    base_type_id: AItemId,
    mutator: RcMuta,
    merged_attrs: RMap<RAttrKey, AAttrVal>,
    merged_effdatas: Option<RMap<REffectKey, RItemEffectData>>,
    axt: RItemAXt,
}
impl ItemMutationDataCache {
    pub(crate) fn get_base_type_id(&self) -> AItemId {
        self.base_type_id
    }
    pub(crate) fn get_r_mutator(&self) -> &RMuta {
        &self.mutator
    }
}

fn convert_request_to_data(mutation_request: ItemMutationRequest) -> ItemMutationData {
    ItemMutationData::new_with_attrs(
        mutation_request.mutator_id,
        mutation_request
            .attrs
            .into_iter()
            .filter_map(|attr_mutation| attr_mutation.value.map(|roll| (attr_mutation.attr_id, roll)))
            .collect(),
    )
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// Attribute mutations
////////////////////////////////////////////////////////////////////////////////////////////////////
fn apply_attr_mutations(
    attrs: &mut RMap<RAttrKey, AAttrVal>,
    mutator: &RMuta,
    attr_rolls: &RMap<AAttrId, UnitInterval>,
    src: &Src,
) {
    for (&attr_key, attr_mutation_range) in mutator.attr_mods.iter() {
        let unmutated_value = match attrs.get(&attr_key) {
            Some(unmutated_value) => *unmutated_value,
            None => continue,
        };
        let attr_id = src.get_attr(attr_key).id;
        match attr_rolls.get(&attr_id) {
            Some(attr_roll) => {
                let mutated_val = mutate_attr_value(unmutated_value, attr_mutation_range, *attr_roll);
                attrs.insert(attr_key, mutated_val);
            }
            // When no roll is defined by user, still limit possible values by what roll range is
            None => {
                let mutated_val = limit_attr_value(unmutated_value, attr_mutation_range);
                attrs.insert(attr_key, mutated_val);
            }
        }
    }
}

fn mutate_attr_value(unmutated_value: AAttrVal, roll_range: &AMutaAttrRange, roll: UnitInterval) -> AAttrVal {
    unmutated_value * (roll_range.min_mult + roll.get_inner() * (roll_range.max_mult - roll_range.min_mult))
}

fn limit_attr_value(unmutated_value: AAttrVal, roll_range: &AMutaAttrRange) -> AAttrVal {
    if roll_range.min_mult >= OF(1.0) {
        return unmutated_value * roll_range.min_mult;
    }
    if roll_range.max_mult <= OF(1.0) {
        return unmutated_value * roll_range.max_mult;
    }
    unmutated_value
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
struct AttrKeyVal {
    key: RAttrKey,
    value: AAttrVal,
}

fn get_combined_attr_value<'a>(
    src: &'a Src,
    base_type_id: &AItemId,
    base_r_item_cache: &mut Option<Option<&'a RcItem>>,
    mutated_r_item: &RItem,
    attr_id: &AAttrId,
) -> Option<AttrKeyVal> {
    let attr_key = src.get_attr_key_by_id(attr_id)?;
    let value = match mutated_r_item.attrs.get(&attr_key) {
        Some(&unmutated_value) => Some(unmutated_value),
        None => match base_r_item_cache {
            Some(opt_base_r_item) => match opt_base_r_item {
                Some(base_r_item) => base_r_item.attrs.get(&attr_key).copied(),
                None => None,
            },
            None => {
                // TODO: if items are moved to slab, get rid of cache logic and just fetch via key
                let opt_base_r_item = src.get_item(base_type_id);
                base_r_item_cache.replace(opt_base_r_item);
                match opt_base_r_item {
                    Some(base_r_item) => base_r_item.attrs.get(&attr_key).copied(),
                    None => None,
                }
            }
        },
    }?;
    Some(AttrKeyVal { key: attr_key, value })
}

fn merge_effect_datas(
    mutated_item: &RItem,
    merged_attrs: &RMap<RAttrKey, AAttrVal>,
    src: &Src,
) -> Option<RMap<REffectKey, RItemEffectData>> {
    let mut result = None;
    let effect_datas = &mutated_item.effect_datas;
    for (&effect_key, effect_data) in effect_datas.iter() {
        let effect = src.get_effect(effect_key);
        // Autocharge - if effect defines autocharge attr ID, and its value references some non-zero
        // type ID, compare it to what's already in effect data; if it's different, create a copy
        // of effect data with new value
        if let Some(charge_info) = &effect.charge
            && let Some(attr_key) = charge_info.location.get_autocharge_attr_key()
        {
            let new_ac_type_id = match merged_attrs.get(&attr_key) {
                Some(&value) => match value.round() as AItemId {
                    0 => None,
                    a_item_id => Some(a_item_id),
                },
                None => None,
            };
            if new_ac_type_id != effect_data.autocharge {
                let inner = result.get_or_insert_with(|| effect_datas.clone());
                inner.get_mut(&effect_key).unwrap().autocharge = new_ac_type_id;
            }
        }
        // Projectee filter - same approach as for autocharges
        if let Some(projectee_filter_info) = &effect.projectee_filter
            && let Some(attr_key) = projectee_filter_info.get_item_list_attr_key()
        {
            let new_projectee_filter = match merged_attrs.get(&attr_key) {
                Some(&value) => match value.round() as AEveItemListId {
                    0 => None,
                    item_list_id => src.get_item_list_key_by_id(&AItemListId::Eve(item_list_id)),
                },
                None => None,
            };
            if new_projectee_filter != effect_data.projectee_filter {
                let inner = result.get_or_insert_with(|| effect_datas.clone());
                inner.get_mut(&effect_key).unwrap().projectee_filter = new_projectee_filter;
            }
        }
    }
    result
}

pub(crate) fn get_combined_attr_values(
    base_r_item: Option<&RcItem>,
    mutated_r_item: &RItem,
) -> RMap<RAttrKey, AAttrVal> {
    match base_r_item {
        Some(base_r_item) => {
            let mut attrs = base_r_item.attrs.clone();
            // Mutated item attributes have priority in case of collisions
            for (&attr_key, &attr_val) in mutated_r_item.attrs.iter() {
                attrs.insert(attr_key, attr_val);
            }
            attrs
        }
        None => mutated_r_item.attrs.clone(),
    }
}

fn make_axt(
    r_item: &RItem,
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    item_effects_override: Option<&RMap<REffectKey, RItemEffectData>>,
    src: &Src,
) -> RItemAXt {
    let mut axt = RItemAXt::default();
    axt.fill(
        r_item.id,
        r_item.grp_id,
        r_item.cat_id,
        item_attrs,
        item_effects_override.unwrap_or(&r_item.effect_datas),
        src.get_attr_id_key_map(),
        src.get_attr_consts(),
        src.get_effect_consts(),
    );
    axt
}
