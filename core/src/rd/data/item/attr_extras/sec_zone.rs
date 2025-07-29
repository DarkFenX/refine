use crate::{
    ac,
    ad::{AAttrId, AAttrVal},
    util::RMap,
};

pub(super) fn is_sec_zone_limitable(item_attrs: &RMap<AAttrId, AAttrVal>) -> bool {
    item_attrs.contains_key(&ac::attrs::DISALLOW_IN_EMPIRE_SPACE)
        || item_attrs.contains_key(&ac::attrs::DISALLOW_IN_HISEC)
        || item_attrs.contains_key(&ac::attrs::DISALLOW_IN_HAZARD)
}
