use crate::{
    UnitInterval, Value,
    ad::{AAttrId, AItemId},
    rd::{RData, RMutaPregenMergedAttrsId},
};

// Expects an effective mutation which can mutate the attribute; returns None if mutation range
// limits the attribute to a single value
pub(in crate::api::item::shared::mutation) fn resolve_absolute_into_roll(
    r_data: &RData,
    pregen_attrs_rid: RMutaPregenMergedAttrsId,
    mutator_type_aid: &AItemId,
    attr_aid: &AAttrId,
    absolute_value: Value,
) -> Option<UnitInterval> {
    let r_mutator = r_data.get_mutator_by_aid(mutator_type_aid).unwrap();
    let unmutated_attrs = r_data.get_muta_pregen().get_merged_attrs(pregen_attrs_rid);
    let attr_rid = r_data.get_attr_rid_by_aid(attr_aid).unwrap();
    let unmutated_value = *unmutated_attrs.get(&attr_rid).unwrap();
    let mutation_range = r_mutator.attr_mods.get(&attr_rid).unwrap();
    let min_value = unmutated_value * mutation_range.mult_min_raw;
    let max_value = unmutated_value * mutation_range.mult_max_raw;
    if min_value == max_value {
        return None;
    }
    let value = (absolute_value - min_value) / (max_value - min_value);
    Some(UnitInterval::from_value_clamped(value))
}
