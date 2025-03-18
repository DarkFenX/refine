use crate::{
    defs::{AttrVal, Count, EAttrId},
    ec,
    util::StMap,
};

pub(super) fn get_volume(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<AttrVal> {
    item_attrs.get(&ec::attrs::VOLUME).copied()
}
pub(super) fn get_bandwidth_use(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<AttrVal> {
    item_attrs.get(&ec::attrs::DRONE_BANDWIDTH_USED).copied()
}
pub(super) fn get_max_type_fitted_count(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<Count> {
    item_attrs.get(&ec::attrs::MAX_TYPE_FITTED).map(|v| v.round() as Count)
}
