use crate::{
    ac,
    ad::{AAttrId, AAttrVal, AItemId},
    ec, ed,
    util::{HMap, HSet},
};

pub(super) fn is_sec_zone_limitable(item_attrs: &HMap<AAttrId, AAttrVal>) -> bool {
    item_attrs.contains_key(&ac::attrs::DISALLOW_IN_EMPIRE_SPACE)
        || item_attrs.contains_key(&ac::attrs::DISALLOW_IN_HISEC)
        || item_attrs.contains_key(&ac::attrs::DISALLOW_IN_HAZARD)
}

pub(super) fn is_disallowed_in_wspace(item_id: &AItemId, type_lists: &HMap<ed::EItemListId, HSet<AItemId>>) -> bool {
    let type_list = match type_lists.get(&ec::itemlists::WORMHOLE_JUMP_BLACK_LIST) {
        Some(type_list) => type_list,
        None => return false,
    };
    type_list.contains(item_id)
}
