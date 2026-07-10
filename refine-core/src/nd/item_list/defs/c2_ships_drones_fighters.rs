use crate::{
    ad::{AItem, AItemCatId, AItemListId},
    nd::NItemList,
};

const ITEM_LIST_AID: AItemListId = AItemListId::SHIPS_DRONES_FIGHTERS;

pub(in crate::nd::item_list) fn mk_n_item_list() -> NItemList {
    NItemList {
        aid: ITEM_LIST_AID,
        adg_item_filter_fn: Some(item_filter),
        ..
    }
}

fn item_filter(a_item: &AItem) -> bool {
    [AItemCatId::SHIP, AItemCatId::DRONE, AItemCatId::FIGHTER].contains(&a_item.cat_id)
}
