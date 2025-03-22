use crate::{
    ac,
    ad::{AAttrId, AAttrVal, ACount},
    util::StMap,
};

pub(super) fn get_volume(item_attrs: &StMap<AAttrId, AAttrVal>) -> Option<AAttrVal> {
    item_attrs.get(&ac::attrs::VOLUME).copied()
}
pub(super) fn get_bandwidth_use(item_attrs: &StMap<AAttrId, AAttrVal>) -> Option<AAttrVal> {
    item_attrs.get(&ac::attrs::DRONE_BANDWIDTH_USED).copied()
}
pub(super) fn get_max_type_fitted_count(item_attrs: &StMap<AAttrId, AAttrVal>) -> Option<ACount> {
    item_attrs.get(&ac::attrs::MAX_TYPE_FITTED).map(|v| v.round() as ACount)
}
pub(super) fn get_online_max_sec_class(item_attrs: &StMap<AAttrId, AAttrVal>) -> Option<AAttrVal> {
    item_attrs.get(&ac::attrs::ONLINE_MAX_SECURITY_CLASS).copied()
}
