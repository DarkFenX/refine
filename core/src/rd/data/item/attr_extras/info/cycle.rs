use crate::{
    num::Value,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn specifies_reactivation_delay(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    match attr_consts.mod_reactivation_delay {
        Some(attr_rid) => item_attrs.contains_key(&attr_rid),
        None => false,
    }
}

pub(in crate::rd::data::item::attr_extras) fn specifies_disallow_repeats(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    match attr_consts.disallow_repeating_activation {
        Some(attr_rid) => item_attrs.contains_key(&attr_rid),
        None => false,
    }
}
