use crate::{
    ad::{AAttrId, AItemId},
    num::{UnitInterval, Value},
    rd::{RAttrId, RMuta, RMutaAttrRange, RcItem, Src},
    ud::{UAttrMutationRequest, get_combined_attr_values},
    util::RMap,
};

pub(in crate::api::item::shared::mutation) fn resolve_absolutes_into_rolls_with_ids(
    src: &Src,
    base_type_aid: &AItemId,
    mutator_type_aid: &AItemId,
    values: &[(AAttrId, Value)],
) -> Vec<UAttrMutationRequest> {
    let Some(r_mutator) = src.get_mutator_by_aid(mutator_type_aid) else {
        return Vec::new();
    };
    let Some(&mutated_type_aid) = r_mutator.item_map.get(base_type_aid) else {
        return Vec::new();
    };
    let base_r_item = src.get_item_by_aid(base_type_aid);
    let mutated_r_item = src.get_item_by_aid(&mutated_type_aid);
    resolve_absolutes_into_rolls_with_items(src, base_r_item, mutated_r_item, r_mutator, values)
}

pub(in crate::api::item::shared::mutation) fn resolve_absolutes_into_rolls_with_items(
    src: &Src,
    base_r_item: Option<&RcItem>,
    mutated_r_item: Option<&RcItem>,
    r_mutator: &RMuta,
    values: &[(AAttrId, Value)],
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
    unmutated_attrs: &RMap<RAttrId, Value>,
    r_mutator: &RMuta,
    values: &[(AAttrId, Value)],
) -> Vec<UAttrMutationRequest> {
    let mut result = Vec::with_capacity(values.len());
    for (attr_aid, absolute_value) in values {
        let Some(attr_rid) = src.get_attr_rid_by_aid(attr_aid) else {
            continue;
        };
        let Some(unmutated_value) = unmutated_attrs.get(&attr_rid) else {
            continue;
        };
        let Some(mutation_range) = r_mutator.attr_mods.get(&attr_rid) else {
            continue;
        };
        if let Some(roll) = resolve_absolute_into_roll(*absolute_value, *unmutated_value, mutation_range) {
            result.push(UAttrMutationRequest {
                attr_aid: *attr_aid,
                roll: Some(roll),
            })
        }
    }
    result
}

fn resolve_absolute_into_roll(
    absolute_value: Value,
    unmutated_value: Value,
    mutation_range: &RMutaAttrRange,
) -> Option<UnitInterval> {
    let min_value = unmutated_value * mutation_range.min_mult;
    let max_value = unmutated_value * mutation_range.max_mult;
    if min_value == max_value {
        return None;
    }
    let value = (absolute_value - min_value) / (max_value - min_value);
    Some(UnitInterval::from_value_clamped(value))
}
