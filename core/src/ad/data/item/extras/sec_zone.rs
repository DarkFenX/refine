use crate::{
    ac,
    ad::{AAttrId, AAttrVal},
    util::StMap,
};

pub(super) fn is_sec_zone_limitable(item_attrs: &StMap<AAttrId, AAttrVal>) -> bool {
    item_attrs.contains_key(&ac::attrs::DISALLOW_IN_EMPIRE_SPACE)
        || item_attrs.contains_key(&ac::attrs::DISALLOW_IN_HISEC)
        || item_attrs.contains_key(&ac::attrs::DISALLOW_IN_HAZARD)
}
