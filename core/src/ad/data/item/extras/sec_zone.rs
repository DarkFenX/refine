use crate::{
    defs::{AttrVal, EAttrId},
    ec,
    util::StMap,
};

pub(super) fn is_sec_zone_limitable(item_attrs: &StMap<EAttrId, AttrVal>) -> bool {
    item_attrs.contains_key(&ec::attrs::DISALLOW_IN_EMPIRE_SPACE)
        || item_attrs.contains_key(&ec::attrs::DISALLOW_IN_HISEC)
        || item_attrs.contains_key(&ec::attrs::DISALLOW_IN_HAZARD)
}
