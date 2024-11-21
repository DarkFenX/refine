use crate::{
    defs::{AttrVal, EAttrId, EItemId, EMutaId, MutaRange},
    err::basic::ItemLoadedError,
    sol::item::{update_a_data_base, SolItemBase},
    src::Src,
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol) struct SolItemMutation {
    // Following fields are part of item skeleton
    pub(in crate::sol::item) base_type_id: EItemId,
    pub(in crate::sol::item) mutator_id: EMutaId,
    pub(in crate::sol::item) mutations: StMap<EAttrId, MutaRange>,
    // Following fields are stored for fast access / optimization
    merged_attrs: Option<StMap<EAttrId, AttrVal>>,
}
impl SolItemMutation {
    pub(in crate::sol::item) fn new(base_type_id: EItemId, mutator_id: EMutaId) -> Self {
        Self {
            base_type_id,
            mutator_id,
            mutations: StMap::new(),
            merged_attrs: None,
        }
    }
}

pub(in crate::sol::item) fn get_attrs_mutated<'a>(
    base: &'a SolItemBase,
    mutation: &'a Option<SolItemMutation>,
) -> Result<&'a StMap<EAttrId, AttrVal>, ItemLoadedError> {
    let mutation = match mutation {
        Some(mutation) => mutation,
        None => return base.get_attrs(),
    };
    match &mutation.merged_attrs {
        Some(merged_attrs) => Ok(merged_attrs),
        None => base.get_attrs(),
    }
}

pub(in crate::sol::item) fn update_a_data_mutated(
    src: &Src,
    base: &mut SolItemBase,
    mutation: &mut Option<SolItemMutation>,
) {
    match mutation {
        Some(mutation) => {
            // If mutation didn't find any data it needed, fall back to base item
            if !update_mutated_a_data_internal(src, base, mutation) {
                base.a_item = src.get_a_item(&mutation.base_type_id).cloned();
                mutation.merged_attrs = None
            }
        }
        None => update_a_data_base(src, base),
    }
}

fn update_mutated_a_data_internal(src: &Src, base: &mut SolItemBase, mutation: &mut SolItemMutation) -> bool {
    let base_a_item = match src.get_a_item(&mutation.base_type_id) {
        Some(base_a_item) => base_a_item,
        None => return false,
    };
    let a_mutator = match src.get_a_muta(&mutation.mutator_id) {
        Some(a_mutator) => a_mutator,
        None => return false,
    };
    let mutated_type_id = match a_mutator.item_map.get(&mutation.base_type_id) {
        Some(mutated_type_id) => *mutated_type_id,
        None => return false,
    };
    let mutated_a_item = match src.get_a_item(&mutated_type_id) {
        Some(mutated_a_item) => mutated_a_item,
        None => return false,
    };
    base.a_item = Some(mutated_a_item.clone());
    // Mutated item attributes have priority
    let mut attrs = base_a_item.attr_vals.clone();
    for (attr_id, attr_val) in mutated_a_item.attr_vals.iter() {
        attrs.insert(*attr_id, *attr_val);
    }
    // Apply mutations
    for (attr_id, attr_roll) in mutation.mutations.iter() {
        let val = match attrs.get(attr_id) {
            Some(val) => *val,
            None => continue,
        };
        if let Some(roll_range) = a_mutator.attr_mods.get(attr_id) {
            let rolled_val = val * (roll_range.min_mult + attr_roll * (roll_range.max_mult - roll_range.min_mult));
            attrs.insert(*attr_id, rolled_val);
        }
    }
    mutation.merged_attrs = Some(attrs);
    true
}
