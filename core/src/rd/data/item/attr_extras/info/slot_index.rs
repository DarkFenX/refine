use crate::{
    ad::{AAttrVal, ASlotIndex},
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn get_implant_slot(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<ASlotIndex> {
    get_slot_from_attr(item_attrs, attr_consts.implantness)
}

pub(in crate::rd::data::item::attr_extras) fn get_booster_slot(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<ASlotIndex> {
    get_slot_from_attr(item_attrs, attr_consts.boosterness)
}

pub(in crate::rd::data::item::attr_extras) fn get_subsystem_slot(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<ASlotIndex> {
    get_slot_from_attr(item_attrs, attr_consts.subsystem_slot)
}

fn get_slot_from_attr(item_attrs: &RMap<RAttrId, AAttrVal>, attr_key: Option<RAttrId>) -> Option<ASlotIndex> {
    attr_key.and_then(|v| item_attrs.get(&v).map(|v| v.round() as ASlotIndex))
}
