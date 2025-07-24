use crate::{ac, ad, util::RMap};

pub(super) fn get_max_fighter_count(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> ad::ACount {
    match item_attrs.get(&ac::attrs::FTR_SQ_MAX_SIZE) {
        // Ensure there can be at least 1 fighter in a squad
        Some(value) => ad::ACount::max(value.round() as ad::ACount, 1),
        None => 1,
    }
}
