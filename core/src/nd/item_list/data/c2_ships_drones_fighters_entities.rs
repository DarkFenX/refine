use crate::{
    ac,
    ad::{AItem, AItemListId},
    nd::NItemList,
};

const A_ITEM_LIST_ID: AItemListId = ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES;

pub(in crate::nd::item_list) fn mk_n_item_list() -> NItemList {
    NItemList {
        eid: None,
        aid: A_ITEM_LIST_ID,
        adg_item_filter_fn: Some(item_filter),
        ..
    }
}

fn item_filter(a_item: &AItem) -> bool {
    [ac::itemcats::SHIP, ac::itemcats::DRONE, ac::itemcats::FIGHTER].contains(&a_item.cat_id)
}
