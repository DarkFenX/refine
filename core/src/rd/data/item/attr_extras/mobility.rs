use crate::{
    ad::AAttrVal,
    def::OF,
    rd::{RAttrConsts, RAttrKey},
    util::RMap,
};

pub(super) fn is_mobile(item_attrs: &RMap<RAttrKey, AAttrVal>, attr_consts: &RAttrConsts) -> bool {
    match attr_consts.max_velocity.and_then(|v| item_attrs.get(&v)) {
        Some(&max_velocity) => max_velocity > OF(0.0001),
        None => false,
    }
}
