use crate::{
    ad::{AItem, AItemId},
    nd::NItem,
};

const ITEM_AID: AItemId = AItemId::ANSIBLEX_JUMP_BRIDGE;

pub(in crate::nd::item) fn mk_n_item() -> NItem {
    NItem {
        aid: ITEM_AID,
        adg_update_item_fn: Some(update_item),
        ..
    }
}

fn update_item(a_item: &mut AItem) {
    a_item.enables_portal = true;
}
