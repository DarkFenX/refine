use crate::{
    defs::{AttrVal, EAttrId},
    ec,
    util::StMap,
};

pub(super) fn get_volume(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<AttrVal> {
    item_attrs.get(&ec::attrs::VOLUME).copied()
}
pub(super) fn get_bandwidth_use(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<AttrVal> {
    item_attrs.get(&ec::attrs::DRONE_BANDWIDTH_USED).copied()
}
