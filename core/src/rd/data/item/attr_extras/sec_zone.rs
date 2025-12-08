use crate::{
    ad::AAttrVal,
    rd::{RAttrConsts, RAttrKey},
    util::RMap,
};

pub(super) fn is_sec_zone_limitable(item_attrs: &RMap<RAttrKey, AAttrVal>, attr_consts: &RAttrConsts) -> bool {
    [
        attr_consts.disallow_in_empire_space,
        attr_consts.disallow_in_hisec,
        attr_consts.disallow_in_hazard,
    ]
    .iter()
    .any(|attr_key| match attr_key {
        Some(attr_key) => item_attrs.contains_key(attr_key),
        None => false,
    })
}
