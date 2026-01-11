use crate::{
    ad::{AItem, AItemGrpId, AItemId, AItemListId},
    nd::NItemList,
};

const A_ITEM_LIST_ID: AItemListId = AItemListId::CAPITALS_FREIGHTERS;

pub(in crate::nd::item_list) fn mk_n_item_list() -> NItemList {
    NItemList {
        eid: None,
        aid: A_ITEM_LIST_ID,
        adg_item_filter_fn: Some(item_filter),
        ..
    }
}

fn item_filter(a_item: &AItem) -> bool {
    [AItemGrpId::FREIGHTER, AItemGrpId::JUMP_FREIGHTER].contains(&a_item.grp_id)
        || a_item.srqs.contains_id(&AItemId::CAPITAL_SHIPS)
}
