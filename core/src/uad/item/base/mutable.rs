use crate::{
    ad,
    def::{ItemId, OF},
    err::basic::ItemNotMutatedError,
    misc::{AttrMutationRequest, EffectMode, ItemMutationRequest},
    src::Src,
    uad::{
        err::ItemMutatedError,
        item::base::{UadEffectUpdates, UadItemBase},
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
pub(in crate::uad::item) struct UadItemBaseMutable {
    base: UadItemBase,
    mutation: MutaOption,
}
impl UadItemBaseMutable {
    pub(in crate::uad::item) fn new(
        item_id: ItemId,
        a_item_id: ad::AItemId,
        a_state: ad::AState,
        mutation_request: Option<ItemMutationRequest>,
        src: &Src,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> Self {
        let mutation_request = match mutation_request {
            Some(mutation_request) => mutation_request,
            // No mutation - regular non-mutated item setup
            None => {
                return Self {
                    base: UadItemBase::new(item_id, a_item_id, a_state, src, reuse_eupdates),
                    mutation: MutaOption::None,
                };
            }
        };
        let mutator_id = mutation_request.mutator_id;
        let mut item_mutation_data = convert_request_to_data(mutation_request);
        let a_mutator = match src.get_a_mutator(&mutator_id) {
            Some(a_mutator) => a_mutator,
            // No mutator - base item with ineffective user-defined mutations
            None => {
                return Self {
                    base: UadItemBase::new(item_id, a_item_id, a_state, src, reuse_eupdates),
                    mutation: MutaOption::Some(item_mutation_data),
                };
            }
        };
        // No mutated item ID in mapping or no mutated item itself
        let mutated_a_item = match a_mutator.item_map.get(&a_item_id).and_then(|v| src.get_a_item(v)) {
            Some(mutated_a_item) => mutated_a_item,
            None => {
                return match src.get_a_item(&a_item_id) {
                    // If base item is available, return base item, but with ineffective
                    // user-defined mutations
                    Some(base_a_item) => Self {
                        base: UadItemBase::base_new_with_a_item(
                            item_id,
                            base_a_item.clone(),
                            a_state,
                            src,
                            reuse_eupdates,
                            None,
                        ),
                        mutation: MutaOption::Some(item_mutation_data),
                    },
                    // No base item - unloaded item with ineffective user-defined mutations
                    None => Self {
                        base: UadItemBase::base_new_with_a_item_id_not_loaded(
                            item_id,
                            a_item_id,
                            a_state,
                            reuse_eupdates,
                        ),
                        mutation: MutaOption::Some(item_mutation_data),
                    },
                };
            }
        };
        // Make proper mutated item once we have all the data
        let mut a_attrs = get_combined_a_attr_values(src.get_a_item(&a_item_id), mutated_a_item);
        let a_item_xt = ad::AItemXt::new_inherited(mutated_a_item, &a_attrs);
        apply_attr_mutations(&mut a_attrs, a_mutator, &item_mutation_data.attr_rolls);
        let regular_base = UadItemBase::base_new_with_a_item(
            item_id,
            mutated_a_item.clone(),
            a_state,
            src,
            reuse_eupdates,
            Some(&a_item_xt),
        );
        item_mutation_data.cache = Some(ItemMutationDataCache {
            base_a_item_id: a_item_id,
            a_mutator: a_mutator.clone(),
            merged_a_attrs: a_attrs,
            a_xt: a_item_xt,
        });
        Self {
            base: regular_base,
            mutation: MutaOption::Some(item_mutation_data),
        }
    }
    // Basic data access methods
    pub(in crate::uad::item) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(in crate::uad::item) fn get_a_item_id(&self) -> ad::AItemId {
        self.base.get_a_item_id()
    }
    pub(in crate::uad::item) fn set_a_item_id(
        &mut self,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        // Since this method is supposed to update base item ID for mutated items, location of ID
        // depends on item configuration
        match &mut self.mutation {
            MutaOption::Some(mutation_data) => match &mut mutation_data.cache {
                Some(mutation_cache) => {
                    mutation_cache.base_a_item_id = a_item_id;
                }
                None => self.base.base_set_a_item_id_primitive(a_item_id),
            },
            MutaOption::None => self.base.base_set_a_item_id_primitive(a_item_id),
        }
        // Even if mutation is not effective with old base type ID, it might become effective with
        // the new one, so - reload adapted data the mutated way regardless of presence of mutation
        // cache
        self.update_a_data(reuse_eupdates, src);
    }
    pub(in crate::uad::item) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_a_group_id()
    }
    pub(in crate::uad::item) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        self.base.get_a_category_id()
    }
    pub(in crate::uad::item) fn get_a_attrs(&self) -> Option<&RMap<ad::AAttrId, ad::AAttrVal>> {
        let item_mutation = match &self.mutation {
            MutaOption::Some(item_mutation) => item_mutation,
            MutaOption::None => return self.base.get_a_attrs(),
        };
        match &item_mutation.cache {
            Some(cache) => Some(&cache.merged_a_attrs),
            None => self.base.get_a_attrs(),
        }
    }
    pub(in crate::uad::item) fn get_a_effect_datas(&self) -> Option<&RMap<ad::AEffectId, ad::AItemEffectData>> {
        self.base.get_a_effect_datas()
    }
    pub(in crate::uad::item) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        self.base.get_a_defeff_id()
    }
    pub(in crate::uad::item) fn get_a_skill_reqs(&self) -> Option<&RMap<ad::AItemId, ad::ASkillLevel>> {
        self.base.get_a_skill_reqs()
    }
    // Extra data access methods
    pub(in crate::uad::item) fn get_a_xt(&self) -> Option<&ad::AItemXt> {
        let item_mutation = match &self.mutation {
            MutaOption::Some(item_mutation) => item_mutation,
            MutaOption::None => return self.base.get_a_xt(),
        };
        match &item_mutation.cache {
            Some(cache) => Some(&cache.a_xt),
            None => self.base.get_a_xt(),
        }
    }
    pub(in crate::uad::item) fn get_max_a_state(&self) -> Option<ad::AState> {
        self.base.base_get_a_item().map(|v| v.ai.max_state)
    }
    pub(in crate::uad::item) fn get_val_fitted_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.base_get_a_item().and_then(|v| v.ai.val_fitted_group_id)
    }
    pub(in crate::uad::item) fn get_val_online_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.base_get_a_item().and_then(|v| v.ai.val_online_group_id)
    }
    pub(in crate::uad::item) fn get_val_active_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.base_get_a_item().and_then(|v| v.ai.val_active_group_id)
    }
    // Misc methods
    pub(in crate::uad::item) fn get_reffs(&self) -> Option<&RSet<ad::AEffectId>> {
        self.base.get_reffs()
    }
    pub(in crate::uad::item) fn start_all_reffs(&self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        self.base.start_all_reffs(reuse_eupdates, src);
    }
    pub(in crate::uad::item) fn stop_all_reffs(&self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        self.base.stop_all_reffs(reuse_eupdates, src);
    }
    pub(in crate::uad::item) fn get_a_state(&self) -> ad::AState {
        self.base.get_a_state()
    }
    pub(in crate::uad::item) fn set_a_state(
        &mut self,
        state: ad::AState,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        self.base
            .base_set_a_state(state, reuse_eupdates, src, self.mutation.get_mutated_a_xt())
    }
    pub(in crate::uad::item) fn get_effect_mode(&self, a_effect_id: &ad::AEffectId) -> EffectMode {
        self.base.get_effect_mode(a_effect_id)
    }
    pub(in crate::uad::item) fn set_effect_mode(
        &mut self,
        a_effect_id: ad::AEffectId,
        effect_mode: EffectMode,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        self.base.base_set_effect_mode(
            a_effect_id,
            effect_mode,
            reuse_eupdates,
            src,
            self.mutation.get_mutated_a_xt(),
        )
    }
    pub(in crate::uad::item) fn set_effect_modes(
        &mut self,
        modes: impl Iterator<Item = (ad::AEffectId, EffectMode)>,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        self.base
            .base_set_effect_modes(modes, reuse_eupdates, src, self.mutation.get_mutated_a_xt())
    }
    pub(in crate::uad::item) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::uad::item) fn update_a_data(&mut self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        let item_mutation = match &mut self.mutation {
            MutaOption::Some(item_mutation) => item_mutation,
            // No mutation - just update base item
            MutaOption::None => {
                self.base.base_update_a_data(reuse_eupdates, src, None);
                return;
            }
        };
        let base_a_item_id = match &item_mutation.cache {
            Some(cache) => cache.base_a_item_id,
            None => self.base.get_a_item_id(),
        };
        let a_mutator = match src.get_a_mutator(&item_mutation.a_mutator_id) {
            Some(a_mutator) => a_mutator,
            // No mutator - invalidate mutated cache and use non-mutated item
            None => match src.get_a_item(&base_a_item_id) {
                Some(base_a_item) => {
                    self.base
                        .base_set_a_item(base_a_item.clone(), reuse_eupdates, src, None);
                    item_mutation.cache = None;
                    return;
                }
                None => {
                    self.base.base_set_a_item_id_not_loaded(base_a_item_id, reuse_eupdates);
                    item_mutation.cache = None;
                    return;
                }
            },
        };
        let mutated_a_item = match a_mutator.item_map.get(&base_a_item_id).and_then(|v| src.get_a_item(v)) {
            Some(mutated_a_item) => mutated_a_item,
            // No mutated aitem ID or no item itself - invalidate mutated cache and use non-mutated
            // item
            None => match src.get_a_item(&base_a_item_id) {
                Some(base_a_item) => {
                    self.base
                        .base_set_a_item(base_a_item.clone(), reuse_eupdates, src, None);
                    item_mutation.cache = None;
                    return;
                }
                None => {
                    self.base.base_set_a_item_id_not_loaded(base_a_item_id, reuse_eupdates);
                    item_mutation.cache = None;
                    return;
                }
            },
        };
        // Compose attribute cache
        let mut a_attrs = get_combined_a_attr_values(src.get_a_item(&base_a_item_id), mutated_a_item);
        let a_xt = ad::AItemXt::new_inherited(mutated_a_item, &a_attrs);
        apply_attr_mutations(&mut a_attrs, a_mutator, &item_mutation.attr_rolls);
        // Everything needed is at hand, update item
        self.base
            .base_set_a_item(mutated_a_item.clone(), reuse_eupdates, src, Some(&a_xt));
        item_mutation.cache = Some(ItemMutationDataCache {
            base_a_item_id,
            a_mutator: a_mutator.clone(),
            merged_a_attrs: a_attrs,
            a_xt,
        })
    }
    // Mutation-specific methods
    pub(in crate::uad::item) fn get_mutation_data(&self) -> Option<&ItemMutationData> {
        self.mutation.as_ref()
    }
    pub(in crate::uad::item) fn mutate(
        &mut self,
        mutation_request: ItemMutationRequest,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) -> Result<(), ItemNotMutatedError> {
        if self.get_mutation_data().is_some() {
            reuse_eupdates.clear();
            return Err(ItemNotMutatedError {
                item_id: self.get_item_id(),
            });
        };
        // Since item is not mutated, base aitem ID is always on non-mutated item base
        let base_a_item_id = self.base.get_a_item_id();
        let mutator_id = mutation_request.mutator_id;
        let mut item_mutation_data = convert_request_to_data(mutation_request);
        let a_mutator = match src.get_a_mutator(&mutator_id) {
            Some(a_mutator) => a_mutator,
            // No mutator - nothing changes, except for user-defined mutations getting stored
            None => {
                self.mutation = MutaOption::Some(item_mutation_data);
                reuse_eupdates.clear();
                return Ok(());
            }
        };
        let mutated_a_item = match a_mutator.item_map.get(&base_a_item_id).and_then(|v| src.get_a_item(v)) {
            Some(mutated_a_item) => mutated_a_item,
            // No mutated aitem ID or no mutated item itself - nothing changes, except for
            // user-defined mutations getting stored
            None => {
                self.mutation = MutaOption::Some(item_mutation_data);
                reuse_eupdates.clear();
                return Ok(());
            }
        };
        // Since we have all the data now, apply mutation properly
        let mut a_attrs = get_combined_a_attr_values(self.base.base_get_a_item(), mutated_a_item);
        let a_xt = ad::AItemXt::new_inherited(mutated_a_item, &a_attrs);
        apply_attr_mutations(&mut a_attrs, a_mutator, &item_mutation_data.attr_rolls);
        self.base
            .base_set_a_item(mutated_a_item.clone(), reuse_eupdates, src, Some(&a_xt));
        item_mutation_data.cache = Some(ItemMutationDataCache {
            base_a_item_id,
            a_mutator: a_mutator.clone(),
            merged_a_attrs: a_attrs,
            a_xt,
        });
        self.mutation = MutaOption::Some(item_mutation_data);
        Ok(())
    }
    pub(in crate::uad::item) fn change_mutation_attrs(
        &mut self,
        src: &Src,
        attr_mutation_requests: Vec<AttrMutationRequest>,
    ) -> Result<Vec<ad::AAttrId>, ItemMutatedError> {
        let item_mutation = match &mut self.mutation {
            MutaOption::Some(item_mutation) => item_mutation,
            MutaOption::None => {
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
                            item_mutation
                                .attr_rolls
                                .insert(attr_mutation_request.a_attr_id, roll_val);
                        }
                        None => {
                            item_mutation.attr_rolls.remove(&attr_mutation_request.a_attr_id);
                        }
                    }
                }
                return Ok(Vec::new());
            }
        };
        // All the methods which set cache guarantee that all the following entities are available
        // for the source the cache was generated with, and this method is supposed to be called
        // with the same source
        let mutated_a_item_id = mutation_cache
            .a_mutator
            .item_map
            .get(&mutation_cache.base_a_item_id)
            .unwrap();
        let mutated_a_item = src.get_a_item(mutated_a_item_id).unwrap();
        // Process mutation requests, recording attributes whose values were changed for the item
        let mut base_a_item_cache = None;
        let mut changed_a_attr_ids = Vec::new();
        for attr_mutation_request in attr_mutation_requests {
            let unmutated_a_value = get_combined_a_attr_value(
                src,
                &mutation_cache.base_a_item_id,
                &mut base_a_item_cache,
                mutated_a_item,
                &attr_mutation_request.a_attr_id,
            );
            let new_a_value = match attr_mutation_request.value {
                // Mutation change request
                Some(attr_roll) => {
                    // Update user-defined data
                    item_mutation
                        .attr_rolls
                        .insert(attr_mutation_request.a_attr_id, attr_roll);
                    // Process source-dependent data and return new value
                    let unmutated_a_value = match unmutated_a_value {
                        Some(unmutated_a_value) => unmutated_a_value,
                        // No unmutated value now means there couldn't be any mutated value with any
                        // mutation earlier as well, thus attribute value cannot change. We already
                        // updated user data, so just go to next attribute
                        None => continue,
                    };
                    let a_mutation_range =
                        match mutation_cache.a_mutator.attr_mods.get(&attr_mutation_request.a_attr_id) {
                            Some(a_mutation_range) => a_mutation_range,
                            // No mutation range now means there couldn't be any mutated value
                            // earlier as well, regardless of user-defined roll data, thus attribute
                            // value cannot change. We already updated user data, so just go to next
                            // attribute
                            None => continue,
                        };
                    mutate_a_attr_value(unmutated_a_value, a_mutation_range, attr_roll)
                }
                // Mutation removal request
                None => {
                    // Update user-defined data
                    item_mutation.attr_rolls.remove(&attr_mutation_request.a_attr_id);
                    // Update source-dependent data
                    let unmutated_a_value = match unmutated_a_value {
                        Some(unmutated_a_value) => unmutated_a_value,
                        // No unmutated value - can't do any comparisons
                        None => continue,
                    };
                    // Limit possible values by roll range, if it is available
                    match mutation_cache.a_mutator.attr_mods.get(&attr_mutation_request.a_attr_id) {
                        Some(a_mutation_range) => limit_a_attr_value(unmutated_a_value, a_mutation_range),
                        None => unmutated_a_value,
                    }
                }
            };
            // Since unmutated value of the attribute is available by now, we can safely assume that
            // merged attributes have some value too (those are supposed to be built using the same
            // logic as unmutated value)
            let old_a_value = mutation_cache
                .merged_a_attrs
                .insert(attr_mutation_request.a_attr_id, new_a_value)
                .unwrap();
            if old_a_value != new_a_value {
                changed_a_attr_ids.push(attr_mutation_request.a_attr_id);
            }
        }
        Ok(changed_a_attr_ids)
    }
    pub(in crate::uad::item) fn set_a_mutator_id(
        &mut self,
        a_mutator_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) -> Result<(), ItemMutatedError> {
        let item_mutation = match &mut self.mutation {
            MutaOption::Some(item_mutation) => item_mutation,
            MutaOption::None => {
                reuse_eupdates.clear();
                return Err(ItemMutatedError {
                    item_id: self.get_item_id(),
                });
            }
        };
        item_mutation.a_mutator_id = a_mutator_id;
        self.update_a_data(reuse_eupdates, src);
        Ok(())
    }
    pub(in crate::uad::item) fn unmutate(
        &mut self,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) -> Result<(), ItemMutatedError> {
        let item_mutation = match &mut self.mutation {
            MutaOption::Some(item_mutation) => item_mutation,
            MutaOption::None => {
                reuse_eupdates.clear();
                return Err(ItemMutatedError {
                    item_id: self.get_item_id(),
                });
            }
        };
        match &item_mutation.cache {
            // If cache is there, mutation is effective - item base has mutated item, and base type
            // ID is stored on cache
            Some(cache) => {
                let a_item_id = cache.base_a_item_id;
                self.base
                    .base_set_a_item_id_and_reload(a_item_id, reuse_eupdates, src, None);
                self.mutation = MutaOption::None;
            }
            // No cache - mutation was not effective, and base item was used already. Just unassign
            // mutation in this case
            None => {
                self.mutation = MutaOption::None;
                reuse_eupdates.clear();
            }
        };
        Ok(())
    }
}

#[derive(Clone)]
enum MutaOption {
    Some(ItemMutationData),
    None,
}
impl MutaOption {
    fn get_mutated_a_xt(&self) -> Option<&ad::AItemXt> {
        match self {
            Self::Some(item_mutation) => item_mutation.cache.as_ref().map(|v| &v.a_xt),
            Self::None => None,
        }
    }
    fn as_ref(&self) -> Option<&ItemMutationData> {
        match self {
            Self::Some(item_mutation) => Some(item_mutation),
            Self::None => None,
        }
    }
}

#[derive(Clone)]
pub(crate) struct ItemMutationData {
    // User-defined data
    a_mutator_id: ad::AItemId,
    attr_rolls: RMap<ad::AAttrId, UnitInterval>,
    // Source-dependent data
    cache: Option<ItemMutationDataCache>,
}
impl ItemMutationData {
    fn new_with_attrs(a_mutator_id: ad::AItemId, attr_rolls: RMap<ad::AAttrId, UnitInterval>) -> Self {
        Self {
            a_mutator_id,
            attr_rolls,
            cache: None,
        }
    }
    pub(crate) fn get_a_mutator_id(&self) -> ad::AItemId {
        self.a_mutator_id
    }
    pub(crate) fn get_attr_rolls(&self) -> &RMap<ad::AAttrId, UnitInterval> {
        &self.attr_rolls
    }
    pub(crate) fn get_cache(&self) -> Option<&ItemMutationDataCache> {
        self.cache.as_ref()
    }
}

// Container for data which is source-dependent
#[derive(Clone)]
pub(crate) struct ItemMutationDataCache {
    base_a_item_id: ad::AItemId,
    a_mutator: ad::ArcMuta,
    merged_a_attrs: RMap<ad::AAttrId, ad::AAttrVal>,
    a_xt: ad::AItemXt,
}
impl ItemMutationDataCache {
    pub(crate) fn get_base_a_item_id(&self) -> ad::AItemId {
        self.base_a_item_id
    }
    pub(crate) fn get_a_mutator(&self) -> &ad::AMuta {
        &self.a_mutator
    }
}

fn convert_request_to_data(mutation_request: ItemMutationRequest) -> ItemMutationData {
    ItemMutationData::new_with_attrs(
        mutation_request.mutator_id,
        mutation_request
            .attrs
            .into_iter()
            .filter_map(|attr_mutation| attr_mutation.value.map(|roll| (attr_mutation.a_attr_id, roll)))
            .collect(),
    )
}

// Attribute mutations
fn apply_attr_mutations(
    a_attrs: &mut RMap<ad::AAttrId, ad::AAttrVal>,
    a_mutator: &ad::AMuta,
    attr_rolls: &RMap<ad::AAttrId, UnitInterval>,
) {
    for (attr_id, attr_mutation_range) in a_mutator.attr_mods.iter() {
        let unmutated_value = match a_attrs.get(attr_id) {
            Some(unmutated_value) => *unmutated_value,
            None => continue,
        };
        match attr_rolls.get(attr_id) {
            Some(attr_roll) => {
                let mutated_val = mutate_a_attr_value(unmutated_value, attr_mutation_range, *attr_roll);
                a_attrs.insert(*attr_id, mutated_val);
            }
            // When no roll is defined by user, still limit possible values by what roll range is
            None => {
                let mutated_val = limit_a_attr_value(unmutated_value, attr_mutation_range);
                a_attrs.insert(*attr_id, mutated_val);
            }
        }
    }
}

fn mutate_a_attr_value(
    unmutated_a_value: ad::AAttrVal,
    roll_range: &ad::AMutaAttrRange,
    roll: UnitInterval,
) -> ad::AAttrVal {
    unmutated_a_value * (roll_range.min_mult + roll.get_inner() * (roll_range.max_mult - roll_range.min_mult))
}

fn limit_a_attr_value(unmutated_a_value: ad::AAttrVal, roll_range: &ad::AMutaAttrRange) -> ad::AAttrVal {
    if roll_range.min_mult >= OF(1.0) {
        return unmutated_a_value * roll_range.min_mult;
    }
    if roll_range.max_mult <= OF(1.0) {
        return unmutated_a_value * roll_range.max_mult;
    }
    unmutated_a_value
}

// Misc functions
fn get_combined_a_attr_value<'a>(
    src: &'a Src,
    base_a_item_id: &ad::AItemId,
    base_a_item_cache: &mut Option<Option<&'a ad::ArcItemRt>>,
    mutated_a_item: &ad::AItemRt,
    a_attr_id: &ad::AAttrId,
) -> Option<ad::AAttrVal> {
    match mutated_a_item.ai.attrs.get(a_attr_id) {
        Some(unmutated_value) => Some(*unmutated_value),
        None => match base_a_item_cache {
            Some(opt_base_a_item) => match opt_base_a_item {
                Some(base_a_item) => base_a_item.ai.attrs.get(a_attr_id).copied(),
                None => None,
            },
            None => {
                let opt_base_a_item = src.get_a_item(base_a_item_id);
                base_a_item_cache.replace(opt_base_a_item);
                match opt_base_a_item {
                    Some(base_a_item) => base_a_item.ai.attrs.get(a_attr_id).copied(),
                    None => None,
                }
            }
        },
    }
}

pub(crate) fn get_combined_a_attr_values(
    base_a_item: Option<&ad::ArcItemRt>,
    mutated_a_item: &ad::AItemRt,
) -> RMap<ad::AAttrId, ad::AAttrVal> {
    match base_a_item {
        Some(base_a_item) => {
            let mut attrs = base_a_item.ai.attrs.clone();
            // Mutated item attributes have priority in case of collisions
            for (attr_id, attr_val) in mutated_a_item.ai.attrs.iter() {
                attrs.insert(*attr_id, *attr_val);
            }
            attrs
        }
        None => mutated_a_item.ai.attrs.clone(),
    }
}
