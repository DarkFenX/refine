use crate::{
    defs::{AttrVal, EAttrId, SlotIndex},
    ec,
    util::StMap,
};

pub(super) fn get_implant_slot(attrs: &StMap<EAttrId, AttrVal>) -> Option<SlotIndex> {
    get_slot_from_attr(attrs, &ec::attrs::IMPLANTNESS)
}

pub(super) fn get_booster_slot(attrs: &StMap<EAttrId, AttrVal>) -> Option<SlotIndex> {
    get_slot_from_attr(attrs, &ec::attrs::BOOSTERNESS)
}

pub(super) fn get_subsystem_slot(attrs: &StMap<EAttrId, AttrVal>) -> Option<SlotIndex> {
    get_slot_from_attr(attrs, &ec::attrs::SUBSYSTEM_SLOT)
}

fn get_slot_from_attr(attrs: &StMap<EAttrId, AttrVal>, attr_id: &EAttrId) -> Option<SlotIndex> {
    attrs.get(attr_id).map(|v| v.round() as SlotIndex)
}
