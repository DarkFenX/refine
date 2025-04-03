use crate::{
    ac,
    ad::{AAttrId, AAttrVal, ACount},
    util::HMap,
};

pub(super) fn get_max_fighter_count(item_attrs: &HMap<AAttrId, AAttrVal>) -> ACount {
    match item_attrs.get(&ac::attrs::FTR_SQ_MAX_SIZE) {
        // Ensure there can be at least 1 fighter in a squad
        Some(value) => ACount::max(value.round() as ACount, 1),
        None => 1,
    }
}
