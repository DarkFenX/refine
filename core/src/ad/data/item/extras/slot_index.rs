use crate::{
    defs::{AttrVal, EAttrId, SlotIndex},
    ec,
    util::StMap,
};

pub(super) fn get_implant_slot(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<SlotIndex> {
    get_slot_from_attr(item_attrs, &ec::attrs::IMPLANTNESS)
}

pub(super) fn get_booster_slot(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<SlotIndex> {
    get_slot_from_attr(item_attrs, &ec::attrs::BOOSTERNESS)
}

pub(super) fn get_subsystem_slot(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<SlotIndex> {
    get_slot_from_attr(item_attrs, &ec::attrs::SUBSYSTEM_SLOT)
}

fn get_slot_from_attr(item_attrs: &StMap<EAttrId, AttrVal>, attr_id: &EAttrId) -> Option<SlotIndex> {
    item_attrs.get(attr_id).map(|v| v.round() as SlotIndex)
}
