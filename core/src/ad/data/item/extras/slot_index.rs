use crate::{
    ac,
    ad::{AAttrId, AAttrVal, ASlotIndex},
    util::RMap,
};

pub(super) fn get_implant_slot(item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<ASlotIndex> {
    get_slot_from_attr(item_attrs, &ac::attrs::IMPLANTNESS)
}

pub(super) fn get_booster_slot(item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<ASlotIndex> {
    get_slot_from_attr(item_attrs, &ac::attrs::BOOSTERNESS)
}

pub(super) fn get_subsystem_slot(item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<ASlotIndex> {
    get_slot_from_attr(item_attrs, &ac::attrs::SUBSYSTEM_SLOT)
}

fn get_slot_from_attr(item_attrs: &RMap<AAttrId, AAttrVal>, attr_id: &AAttrId) -> Option<ASlotIndex> {
    item_attrs.get(attr_id).map(|v| v.round() as ASlotIndex)
}
