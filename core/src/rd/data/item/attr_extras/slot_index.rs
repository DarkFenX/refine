use crate::{ac, ad, util::RMap};

pub(super) fn get_implant_slot(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> Option<ad::ASlotIndex> {
    get_slot_from_attr(item_attrs, &ac::attrs::IMPLANTNESS)
}

pub(super) fn get_booster_slot(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> Option<ad::ASlotIndex> {
    get_slot_from_attr(item_attrs, &ac::attrs::BOOSTERNESS)
}

pub(super) fn get_subsystem_slot(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> Option<ad::ASlotIndex> {
    get_slot_from_attr(item_attrs, &ac::attrs::SUBSYSTEM_SLOT)
}

fn get_slot_from_attr(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>, attr_id: &ad::AAttrId) -> Option<ad::ASlotIndex> {
    item_attrs.get(attr_id).map(|v| v.round() as ad::ASlotIndex)
}
