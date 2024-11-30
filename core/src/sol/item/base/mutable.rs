use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, EItemGrpId, EItemId, EMutaId, MutaRange, SkillLevel, SolItemId},
    err::basic::{ItemLoadedError, ItemMutatedError, ItemNotMutatedError},
    sol::item::{SolEffectModes, SolItemAttrMutation, SolItemBase, SolItemMutation, SolItemState},
    src::Src,
    util::StMap,
};

// Item mutable base stores all the data every mutable item should have.
//
// Mutation of an item can have 2 states:
// - Loaded - source had all the needed data which was stored on cache. In this case, item base
// stores mutated item type, and base item type ID is stored on mutation cache;
// - Not loaded - item base stores base item type.
#[derive(Clone)]
pub(in crate::sol::item) struct SolItemBaseMutable {
    base: SolItemBase,
    mutation: Option<SolItemMutationData>,
}
impl SolItemBaseMutable {
    pub(in crate::sol::item) fn new(
        src: &Src,
        id: SolItemId,
        type_id: EItemId,
        state: SolItemState,
        mutation_request: Option<SolItemMutation>,
    ) -> Self {
        let mutation_request = match mutation_request {
            Some(mutation_request) => mutation_request,
            // No mutation - regular non-mutated item setup
            None => {
                return Self {
                    base: SolItemBase::new(src, id, type_id, state),
                    mutation: None,
                }
            }
        };
        let base_a_item = match src.get_a_item(&type_id) {
            Some(base_a_item) => base_a_item,
            // No base item - base unloaded item with basic mutation data
            None => {
                return Self {
                    base: SolItemBase::new_with_id_unloaded(id, type_id, state),
                    mutation: Some(convert_basic(mutation_request)),
                }
            }
        };
        let a_mutator = match src.get_a_muta(&mutation_request.mutator_id) {
            Some(a_mutator) => a_mutator,
            // No mutator - base loaded item with basic mutation data
            None => {
                return Self {
                    base: SolItemBase::new_with_item(id, base_a_item.clone(), state),
                    mutation: Some(convert_basic(mutation_request)),
                }
            }
        };
        let mutated_type_id = match a_mutator.item_map.get(&type_id) {
            Some(mutated_type_id) => *mutated_type_id,
            // No mutated item type ID - base item, but with more mutation data. Unlike on previous
            // steps, here it's possible to convert absolute mutated attribute values into ranges
            // using base item attributes as base values
            None => {
                return Self {
                    base: SolItemBase::new_with_item(id, base_a_item.clone(), state),
                    mutation: Some(convert_full(mutation_request, &base_a_item.attr_vals, a_mutator)),
                }
            }
        };
        let mutated_a_item = match src.get_a_item(&mutated_type_id) {
            Some(mutated_a_item) => mutated_a_item,
            // No mutated item - same as previous step, i.e. base item, but with more mutation data
            None => {
                return Self {
                    base: SolItemBase::new_with_item(id, base_a_item.clone(), state),
                    mutation: Some(convert_full(mutation_request, &base_a_item.attr_vals, a_mutator)),
                }
            }
        };
        // Make proper mutated item once we have all the data
        let mut attrs = merge_attrs(base_a_item, mutated_a_item);
        let mut mutation = convert_full(mutation_request, &attrs, a_mutator);
        apply_attr_mutations(&mut attrs, a_mutator, &mutation.attr_ranges);
        mutation.cache = Some(SolItemMutationDataCache::new(type_id, attrs));
        Self {
            base: SolItemBase::new_with_item(id, base_a_item.clone(), state),
            mutation: Some(mutation),
        }
    }
    pub(in crate::sol::item) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol::item) fn get_type_id(&self) -> EItemId {
        self.base.get_type_id()
    }
    pub(in crate::sol::item) fn get_group_id(&self) -> Result<EItemGrpId, ItemLoadedError> {
        self.base.get_group_id()
    }
    pub(in crate::sol::item) fn get_category_id(&self) -> Result<EItemGrpId, ItemLoadedError> {
        self.base.get_category_id()
    }
    pub(in crate::sol::item) fn get_attrs(&self) -> Result<&StMap<EAttrId, AttrVal>, ItemLoadedError> {
        let mutation = match &self.mutation {
            Some(mutation) => mutation,
            None => return self.base.get_attrs(),
        };
        match &mutation.cache {
            Some(cache) => Ok(&cache.merged_attrs),
            None => self.base.get_attrs(),
        }
    }
    pub(in crate::sol::item) fn get_effect_datas(
        &self,
    ) -> Result<&StMap<EEffectId, ad::AItemEffectData>, ItemLoadedError> {
        self.base.get_effect_datas()
    }
    pub(in crate::sol::item) fn get_defeff_id(&self) -> Result<Option<EEffectId>, ItemLoadedError> {
        self.base.get_defeff_id()
    }
    pub(in crate::sol::item) fn get_skill_reqs(&self) -> Result<&StMap<EItemId, SkillLevel>, ItemLoadedError> {
        self.base.get_skill_reqs()
    }
    pub(in crate::sol::item) fn get_state(&self) -> SolItemState {
        self.base.get_state()
    }
    pub(in crate::sol::item) fn set_state(&mut self, state: SolItemState) {
        self.base.set_state(state)
    }
    pub(in crate::sol::item) fn get_effect_modes(&self) -> &SolEffectModes {
        self.base.get_effect_modes()
    }
    pub(in crate::sol::item) fn get_effect_modes_mut(&mut self) -> &mut SolEffectModes {
        self.base.get_effect_modes_mut()
    }
    pub(in crate::sol::item) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::sol::item) fn update_a_data(&mut self, src: &Src) {
        let mutation = match &mut self.mutation {
            Some(mutation) => mutation,
            // No mutation - just update base item
            None => {
                self.base.update_a_data(src);
                return;
            }
        };
        let base_type_id = match &mutation.cache {
            Some(cache) => cache.base_type_id,
            None => self.base.get_type_id(),
        };
        let base_a_item = match src.get_a_item(&base_type_id) {
            Some(base_a_item) => base_a_item,
            // No base item - invalidate mutated cache and use base item data we have, i.e. just ID
            None => {
                self.base.set_type_id(base_type_id);
                self.base.remove_a_item();
                mutation.cache = None;
                return;
            }
        };
        let a_mutator = match src.get_a_muta(&mutation.mutator_id) {
            Some(a_mutator) => a_mutator,
            // No mutator - invalidate mutated cache and use non-mutated item
            None => {
                self.base.set_type_id(base_type_id);
                self.base.set_a_item(base_a_item.clone());
                mutation.cache = None;
                return;
            }
        };
        let mutated_type_id = match a_mutator.item_map.get(&base_type_id) {
            Some(mutated_type_id) => *mutated_type_id,
            // No mutated item type ID - invalidate mutated cache and use non-mutated item
            None => {
                self.base.set_type_id(base_type_id);
                self.base.set_a_item(base_a_item.clone());
                mutation.cache = None;
                return;
            }
        };
        let mutated_a_item = match src.get_a_item(&mutated_type_id) {
            Some(mutated_a_item) => mutated_a_item,
            // No mutated item - invalidate mutated cache and use non-mutated item
            None => {
                self.base.set_type_id(base_type_id);
                self.base.set_a_item(base_a_item.clone());
                mutation.cache = None;
                return;
            }
        };
        // Compose attribute cache
        let mut attrs = merge_attrs(base_a_item, mutated_a_item);
        apply_attr_mutations(&mut attrs, a_mutator, &mutation.attr_ranges);
        // Everything needed is at hand, update item
        self.base.set_type_id(mutated_type_id);
        self.base.set_a_item(mutated_a_item.clone());
        mutation.cache = Some(SolItemMutationDataCache::new(base_type_id, attrs))
    }
    // Mutation-specific methods
    pub(in crate::sol::item) fn mutate(
        &mut self,
        src: &Src,
        mutation_request: SolItemMutation,
    ) -> Result<(), ItemNotMutatedError> {
        if self.mutation.is_some() {
            return Err(ItemNotMutatedError::new(self.get_id()));
        };
        // Since item is not mutated, base type ID is always on non-mutated item base
        let base_type_id = self.base.get_type_id();
        let base_a_item = match self.base.get_a_item() {
            Ok(base_a_item) => base_a_item,
            // No base item - apply only basic mutation data
            Err(_) => {
                self.mutation = Some(convert_basic(mutation_request));
                return Ok(());
            }
        };
        let a_mutator = match src.get_a_muta(&mutation_request.mutator_id) {
            Some(a_mutator) => a_mutator,
            // No mutator - apply only basic mutation data
            None => {
                self.mutation = Some(convert_basic(mutation_request));
                return Ok(());
            }
        };
        let mutated_type_id = match a_mutator.item_map.get(&base_type_id) {
            Some(mutated_type_id) => *mutated_type_id,
            // No mutated item type ID - extended mutation data. Unlike on previous steps, here it's
            // possible to convert absolute mutated attribute values into ranges using base item
            // attributes as base values
            None => {
                self.mutation = Some(convert_full(mutation_request, &base_a_item.attr_vals, a_mutator));
                return Ok(());
            }
        };
        let mutated_a_item = match src.get_a_item(&mutated_type_id) {
            Some(mutated_a_item) => mutated_a_item,
            // No mutated item - same as previous step, i.e. extended mutation data
            None => {
                self.mutation = Some(convert_full(mutation_request, &base_a_item.attr_vals, a_mutator));
                return Ok(());
            }
        };
        // Since we have all the data now, apply mutation properly
        let mut attrs = merge_attrs(base_a_item, mutated_a_item);
        let mut mutation = convert_full(mutation_request, &attrs, a_mutator);
        apply_attr_mutations(&mut attrs, a_mutator, &mutation.attr_ranges);
        mutation.cache = Some(SolItemMutationDataCache::new(base_type_id, attrs));
        self.base.set_type_id(mutated_type_id);
        self.base.set_a_item(mutated_a_item.clone());
        self.mutation = Some(mutation);
        Ok(())
    }
    pub(in crate::sol::item) fn change_mutation(&mut self) {}
    pub(in crate::sol::item) fn unmutate(&mut self, src: &Src) -> Result<(), ItemMutatedError> {
        let mutation = match &mut self.mutation {
            Some(mutation) => mutation,
            None => return Err(ItemMutatedError::new(self.get_id())),
        };
        match &mutation.cache {
            // If cache is valid, base type ID is stored there
            Some(cache) => {
                let type_id = cache.base_type_id;
                self.base.set_type_id_and_reload(src, type_id);
                self.mutation = None;
            }
            // No cache - mutation was not effective, and base item was used already. Just unassign
            // mutation in this case
            None => self.mutation = None,
        };
        Ok(())
    }
}

#[derive(Clone)]
struct SolItemMutationData {
    // User-defined data
    mutator_id: EMutaId,
    attr_ranges: StMap<EAttrId, MutaRange>,
    // Source-dependent data
    cache: Option<SolItemMutationDataCache>,
}
impl SolItemMutationData {
    fn new(mutator_id: EMutaId) -> Self {
        Self {
            mutator_id,
            attr_ranges: StMap::new(),
            cache: None,
        }
    }
    fn new_with_attrs(mutator_id: EMutaId, attr_ranges: StMap<EAttrId, MutaRange>) -> Self {
        Self {
            mutator_id,
            attr_ranges,
            cache: None,
        }
    }
}

// Container for data which is source-dependent
#[derive(Clone)]
struct SolItemMutationDataCache {
    base_type_id: EItemId,
    merged_attrs: StMap<EAttrId, AttrVal>,
}
impl SolItemMutationDataCache {
    fn new(base_type_id: EItemId, merged_attrs: StMap<EAttrId, AttrVal>) -> Self {
        Self {
            base_type_id,
            merged_attrs,
        }
    }
}

fn merge_attrs(base_a_item: &ad::AItem, mutated_a_item: &ad::AItem) -> StMap<EAttrId, AttrVal> {
    let mut attrs = base_a_item.attr_vals.clone();
    // Mutated item attributes have priority in case of collisions
    for (attr_id, attr_val) in mutated_a_item.attr_vals.iter() {
        attrs.insert(*attr_id, *attr_val);
    }
    attrs
}

fn apply_attr_mutations(
    attrs: &mut StMap<EAttrId, AttrVal>,
    a_mutator: &ad::AMuta,
    attr_ranges: &StMap<EAttrId, MutaRange>,
) {
    for (attr_id, attr_roll) in attr_ranges.iter() {
        let val = match attrs.get(attr_id) {
            Some(val) => *val,
            None => continue,
        };
        if let Some(roll_range) = a_mutator.attr_mods.get(attr_id) {
            let rolled_val = val * (roll_range.min_mult + attr_roll * (roll_range.max_mult - roll_range.min_mult));
            attrs.insert(*attr_id, rolled_val);
        }
    }
}

fn convert_basic(mutation_request: SolItemMutation) -> SolItemMutationData {
    SolItemMutationData::new_with_attrs(
        mutation_request.mutator_id,
        mutation_request
            .attrs
            .into_iter()
            .filter_map(|(k, v)| match v {
                SolItemAttrMutation::Range(range) => Some((k, range)),
                // Cannot interpret mutated absolute value without extra context
                SolItemAttrMutation::Value(_) => None,
            })
            .collect(),
    )
}

fn convert_full(
    mutation_request: SolItemMutation,
    base_attrs: &StMap<EAttrId, AttrVal>,
    a_mutator: &ad::AMuta,
) -> SolItemMutationData {
    SolItemMutationData::new_with_attrs(
        mutation_request.mutator_id,
        mutation_request
            .attrs
            .into_iter()
            .filter_map(|(k, v)| normalize_mutated_attr_val_full(base_attrs, a_mutator, &k, v).map(|v| (k, v)))
            .collect(),
    )
}

fn normalize_mutated_attr_val_full(
    base_attrs: &StMap<EAttrId, AttrVal>,
    a_mutator: &ad::AMuta,
    attr_id: &EAttrId,
    value: SolItemAttrMutation,
) -> Option<MutaRange> {
    match value {
        SolItemAttrMutation::Range(range) => Some(range),
        SolItemAttrMutation::Value(abs_value) => {
            let base_value = match base_attrs.get(attr_id) {
                Some(v) => *v,
                None => return None,
            };
            let (min_mult, max_mult) = match a_mutator.attr_mods.get(attr_id) {
                Some(r) => (r.min_mult, r.max_mult),
                None => return None,
            };
            let min_value = base_value * min_mult;
            let max_value = base_value * max_mult;
            Some((abs_value - min_value) / (max_value - min_value))
        }
    }
}
