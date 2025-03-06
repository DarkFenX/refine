use crate::{
    defs::{AttrVal, Count, EAttrId},
    ec,
    util::StMap,
};

pub(super) fn get_max_fighter_count(item_attrs: &StMap<EAttrId, AttrVal>) -> Count {
    match item_attrs.get(&ec::attrs::FTR_SQ_MAX_SIZE) {
        // Ensure there can be at least 1 fighter in a squad
        Some(value) => Count::max(value.round() as Count, 1),
        None => 1,
    }
}
