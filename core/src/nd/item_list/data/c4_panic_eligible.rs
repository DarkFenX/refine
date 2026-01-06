// Items belonging to this list can receive PANIC buffs.

use crate::{
    ac,
    ad::{AItem, AItemListId},
    nd::NItemList,
};

const A_ITEM_LIST_ID: AItemListId = AItemListId::PANIC_ELIGIBLE;

pub(in crate::nd::item_list) fn mk_n_item_list() -> NItemList {
    NItemList {
        eid: None,
        aid: A_ITEM_LIST_ID,
        adg_item_filter_fn: Some(item_filter),
        ..
    }
}

fn item_filter(a_item: &AItem) -> bool {
    [
        AItemGrpId::HAULER,
        AItemGrpId::BLOCKADE_RUNNER,
        AItemGrpId::DEEP_SPACE_TRANSPORT,
        AItemGrpId::EXPEDITION_FRIGATE,
        AItemGrpId::MINING_BARGE,
        AItemGrpId::EXHUMER,
        AItemGrpId::INDUSTRIAL_COMMAND_SHIP,
    ]
    .contains(&a_item.grp_id)
        || [
            AItemId::VENTURE,
            AItemId::VENTURE_CONSORTIUM_ISSUE,
            AItemId::PIONEER,
            AItemId::PIONEER_CONSORTIUM_ISSUE,
            AItemId::OUTRIDER,
        ]
        .contains(&a_item.id)
}
