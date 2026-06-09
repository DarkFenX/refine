use crate::{
    ad::{AItem, AItemGrpId, AItemId, AItemListId},
    nd::NItemList,
};

const ITEM_LIST_AID: AItemListId = AItemListId::CAPITALS_FREIGHTERS;

pub(in crate::nd::item_list) fn mk_n_item_list() -> NItemList {
    NItemList {
        aid: ITEM_LIST_AID,
        adg_item_filter_fn: Some(item_filter),
        ..
    }
}

fn item_filter(a_item: &AItem) -> bool {
    [AItemGrpId::FREIGHTER, AItemGrpId::JUMP_FREIGHTER].contains(&a_item.grp_id)
        || a_item.srqs.contains_id(&AItemId::CAPITAL_SHIPS)
}
