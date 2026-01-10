use crate::{
    num::{SlotIndex, Value},
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn get_implant_slot(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<SlotIndex> {
    get_slot_from_attr(item_attrs, attr_consts.implantness)
}

pub(in crate::rd::data::item::attr_extras) fn get_booster_slot(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<SlotIndex> {
    get_slot_from_attr(item_attrs, attr_consts.boosterness)
}

pub(in crate::rd::data::item::attr_extras) fn get_subsystem_slot(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<SlotIndex> {
    get_slot_from_attr(item_attrs, attr_consts.subsystem_slot)
}

fn get_slot_from_attr(item_attrs: &RMap<RAttrId, Value>, attr_key: Option<RAttrId>) -> Option<SlotIndex> {
    attr_key.and_then(|v| item_attrs.get(&v).map(|v| SlotIndex::from_f64_rounded(v.into_f64())))
}
