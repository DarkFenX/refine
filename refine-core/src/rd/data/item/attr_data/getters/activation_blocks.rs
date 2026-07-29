use crate::{
    Value,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::attr_data) fn get_activation_blocks_cloak(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    match attr_consts.can_cloak.and_then(|v| item_attrs.get(&v)) {
        Some(&value) => !value.is_flag_set(),
        None => false,
    }
}

pub(in crate::rd::data::item::attr_data) fn get_activation_blocks_in_assist(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    match attr_consts.disallow_assistance.and_then(|v| item_attrs.get(&v)) {
        Some(&value) => value.is_flag_set(),
        None => false,
    }
}
