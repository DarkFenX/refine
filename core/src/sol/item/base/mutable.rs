use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, EItemGrpId, EItemId, EMutaId, MutaRange, SkillLevel, SolItemId},
    err::basic::ItemLoadedError,
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
        mutation: Option<SolItemMutation>,
    ) -> Self {
        Self {
            base: SolItemBase::new(src, id, type_id, state),
            mutation: None,
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
        // Compose attribute cache - mutated item attributes have priority
        let mut attrs = base_a_item.attr_vals.clone();
        for (attr_id, attr_val) in mutated_a_item.attr_vals.iter() {
            attrs.insert(*attr_id, *attr_val);
        }
        // Compose attribute cache - apply mutations
        for (attr_id, attr_roll) in mutation.attr_ranges.iter() {
            let val = match attrs.get(attr_id) {
                Some(val) => *val,
                None => continue,
            };
            if let Some(roll_range) = a_mutator.attr_mods.get(attr_id) {
                let rolled_val = val * (roll_range.min_mult + attr_roll * (roll_range.max_mult - roll_range.min_mult));
                attrs.insert(*attr_id, rolled_val);
            }
        }
        // Everything needed is at hand, update item
        self.base.set_type_id(mutated_type_id);
        self.base.set_a_item(mutated_a_item.clone());
        mutation.cache = Some(SolItemMutationDataCache::new(base_type_id, attrs))
    }
    // Mutation-specific methods
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

fn normalize_mutated_attr_val(
    attr_id: &EAttrId,
    value: SolItemAttrMutation,
    base_item: &ad::AItem,
    mutator: &ad::AMuta,
) -> Option<MutaRange> {
    match value {
        SolItemAttrMutation::Range(range) => Some(range),
        SolItemAttrMutation::Value(abs_value) => {
            let base_value = match base_item.attr_vals.get(attr_id) {
                Some(v) => *v,
                None => return None,
            };
            let (min_mult, max_mult) = match mutator.attr_mods.get(attr_id) {
                Some(r) => (r.min_mult, r.max_mult),
                None => return None,
            };
            let min_value = base_value * min_mult;
            let max_value = base_value * max_mult;
            Some((abs_value - min_value) / (max_value - min_value))
        }
    }
}
