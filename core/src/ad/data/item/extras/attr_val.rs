use crate::{
    ac,
    ad::{AAttrId, AAttrVal, ACount},
    def::OF,
    util::RMap,
};

pub(super) fn get_volume(item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<AAttrVal> {
    item_attrs.get(&ac::attrs::VOLUME).copied()
}
pub(super) fn get_bandwidth_use(item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<AAttrVal> {
    item_attrs.get(&ac::attrs::DRONE_BANDWIDTH_USED).copied()
}
pub(super) fn get_max_type_fitted_count(item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<ACount> {
    item_attrs.get(&ac::attrs::MAX_TYPE_FITTED).map(|v| v.round() as ACount)
}
pub(super) fn get_online_max_sec_class(item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<AAttrVal> {
    item_attrs.get(&ac::attrs::ONLINE_MAX_SECURITY_CLASS).copied()
}
pub(super) fn get_remote_resist_attr_id(item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<AAttrId> {
    item_attrs
        .get(&ac::attrs::REMOTE_RESISTANCE_ID)
        .and_then(|val| match val {
            OF(0.0) => None,
            _ => Some(val.into_inner().round() as AAttrId),
        })
}
