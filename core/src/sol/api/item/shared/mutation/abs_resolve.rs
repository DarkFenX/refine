use crate::{
    ad,
    def::AttrVal,
    misc::AttrMutationRequest,
    rd,
    src::Src,
    uad::get_combined_attr_values,
    util::{RMap, UnitInterval},
};

pub(in crate::sol::api::item::shared::mutation) fn resolve_absolutes_into_rolls_with_ids(
    src: &Src,
    base_a_item_id: &ad::AItemId,
    a_mutator_id: &ad::AItemId,
    values: &[(ad::AAttrId, AttrVal)],
) -> Vec<AttrMutationRequest> {
    let r_mutator = match src.get_r_mutator(a_mutator_id) {
        Some(a_mutator) => a_mutator,
        None => return Vec::new(),
    };
    let mutated_a_item_id = match r_mutator.get_item_map().get(base_a_item_id) {
        Some(mutated_a_item_id) => *mutated_a_item_id,
        None => return Vec::new(),
    };
    let base_r_item = src.get_r_item(base_a_item_id);
    let mutated_r_item = src.get_r_item(&mutated_a_item_id);
    resolve_absolutes_into_rolls_with_items(base_r_item, mutated_r_item, r_mutator, values)
}

pub(in crate::sol::api::item::shared::mutation) fn resolve_absolutes_into_rolls_with_items(
    base_r_item: Option<&rd::RcItem>,
    mutated_r_item: Option<&rd::RcItem>,
    r_mutator: &rd::RMuta,
    values: &[(ad::AAttrId, AttrVal)],
) -> Vec<AttrMutationRequest> {
    match (base_r_item, mutated_r_item) {
        (Some(base_r_item), Some(mutated_r_item)) => {
            let combined_attrs = get_combined_attr_values(Some(base_r_item), mutated_r_item);
            resolve_absolutes_into_rolls_with_attrs(&combined_attrs, r_mutator, values)
        }
        (Some(base_r_item), None) => {
            resolve_absolutes_into_rolls_with_attrs(base_r_item.get_attrs(), r_mutator, values)
        }
        (None, Some(mutated_r_item)) => {
            resolve_absolutes_into_rolls_with_attrs(mutated_r_item.get_attrs(), r_mutator, values)
        }
        (None, None) => Vec::new(),
    }
}

pub(in crate::sol::api::item::shared::mutation) fn resolve_absolutes_into_rolls_with_attrs(
    unmutated_attrs: &RMap<ad::AAttrId, ad::AAttrVal>,
    r_mutator: &rd::RMuta,
    values: &[(ad::AAttrId, AttrVal)],
) -> Vec<AttrMutationRequest> {
    let mut result = Vec::with_capacity(values.len());
    for (a_attr_id, absolute_value) in values {
        let unmutated_a_value = match unmutated_attrs.get(a_attr_id) {
            Some(unmutated_a_value) => unmutated_a_value,
            None => continue,
        };
        let a_mutation_range = match r_mutator.get_attr_mods().get(a_attr_id) {
            Some(a_mutation_range) => a_mutation_range,
            None => continue,
        };
        if let Some(roll) = resolve_absolute_into_roll(*absolute_value, *unmutated_a_value, a_mutation_range) {
            result.push(AttrMutationRequest {
                a_attr_id: *a_attr_id,
                value: Some(roll),
            })
        }
    }
    result
}

fn resolve_absolute_into_roll(
    absolute_value: AttrVal,
    unmutated_a_value: ad::AAttrVal,
    a_mutation_range: &ad::AMutaAttrRange,
) -> Option<UnitInterval> {
    let min_value = unmutated_a_value * a_mutation_range.min_mult;
    let max_value = unmutated_a_value * a_mutation_range.max_mult;
    if min_value == max_value {
        return None;
    }
    let value = (absolute_value - min_value) / (max_value - min_value);
    Some(UnitInterval::new_clamped_of64(value))
}
