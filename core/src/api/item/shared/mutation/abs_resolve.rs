use crate::{
    ad::{AAttrId, AAttrVal, AItemId, AMutaAttrRange},
    def::AttrVal,
    rd::{RAttrId, RMuta, RcItem, Src},
    ud::{UAttrMutationRequest, get_combined_attr_values},
    util::{RMap, UnitInterval},
};

pub(in crate::api::item::shared::mutation) fn resolve_absolutes_into_rolls_with_ids(
    src: &Src,
    base_type_id: &AItemId,
    mutator_id: &AItemId,
    values: &[(AAttrId, AttrVal)],
) -> Vec<UAttrMutationRequest> {
    let r_mutator = match src.get_mutator_by_aid(mutator_id) {
        Some(a_mutator) => a_mutator,
        None => return Vec::new(),
    };
    let mutated_type_id = match r_mutator.item_map.get(base_type_id) {
        Some(&mutated_type_id) => mutated_type_id,
        None => return Vec::new(),
    };
    let base_r_item = src.get_item_by_aid(base_type_id);
    let mutated_r_item = src.get_item_by_aid(&mutated_type_id);
    resolve_absolutes_into_rolls_with_items(src, base_r_item, mutated_r_item, r_mutator, values)
}

pub(in crate::api::item::shared::mutation) fn resolve_absolutes_into_rolls_with_items(
    src: &Src,
    base_r_item: Option<&RcItem>,
    mutated_r_item: Option<&RcItem>,
    r_mutator: &RMuta,
    values: &[(AAttrId, AttrVal)],
) -> Vec<UAttrMutationRequest> {
    match (base_r_item, mutated_r_item) {
        (Some(base_r_item), Some(mutated_r_item)) => {
            let combined_attrs = get_combined_attr_values(Some(base_r_item), mutated_r_item);
            resolve_absolutes_into_rolls_with_attrs(src, &combined_attrs, r_mutator, values)
        }
        (Some(base_r_item), None) => {
            resolve_absolutes_into_rolls_with_attrs(src, &base_r_item.attrs, r_mutator, values)
        }
        (None, Some(mutated_r_item)) => {
            resolve_absolutes_into_rolls_with_attrs(src, &mutated_r_item.attrs, r_mutator, values)
        }
        (None, None) => Vec::new(),
    }
}

pub(in crate::api::item::shared::mutation) fn resolve_absolutes_into_rolls_with_attrs(
    src: &Src,
    unmutated_attrs: &RMap<RAttrId, AAttrVal>,
    r_mutator: &RMuta,
    values: &[(AAttrId, AttrVal)],
) -> Vec<UAttrMutationRequest> {
    let mut result = Vec::with_capacity(values.len());
    for (attr_aid, absolute_value) in values {
        let attr_key = match src.get_attr_rid_by_aid(attr_aid) {
            Some(attr_key) => attr_key,
            None => continue,
        };
        let unmutated_a_value = match unmutated_attrs.get(&attr_key) {
            Some(unmutated_a_value) => unmutated_a_value,
            None => continue,
        };
        let a_mutation_range = match r_mutator.attr_mods.get(&attr_key) {
            Some(a_mutation_range) => a_mutation_range,
            None => continue,
        };
        if let Some(roll) = resolve_absolute_into_roll(*absolute_value, *unmutated_a_value, a_mutation_range) {
            result.push(UAttrMutationRequest {
                attr_id: *attr_aid,
                value: Some(roll),
            })
        }
    }
    result
}

fn resolve_absolute_into_roll(
    absolute_value: AttrVal,
    unmutated_a_value: AAttrVal,
    a_mutation_range: &AMutaAttrRange,
) -> Option<UnitInterval> {
    let min_value = unmutated_a_value * a_mutation_range.min_mult;
    let max_value = unmutated_a_value * a_mutation_range.max_mult;
    if min_value == max_value {
        return None;
    }
    let value = (absolute_value - min_value) / (max_value - min_value);
    Some(UnitInterval::new_clamped_of64(value))
}
