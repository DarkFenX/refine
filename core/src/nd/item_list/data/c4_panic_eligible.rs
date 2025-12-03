use crate::{
    ac,
    ad::{AItem, AItemListId},
    nd::NItemList,
};

const A_ITEM_LIST_ID: AItemListId = ac::itemlists::PANIC_ELIGIBLE;

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
        ac::itemgrps::HAULER,
        ac::itemgrps::BLOCKADE_RUNNER,
        ac::itemgrps::DEEP_SPACE_TRANSPORT,
        ac::itemgrps::EXPEDITION_FRIGATE,
        ac::itemgrps::MINING_BARGE,
        ac::itemgrps::EXHUMER,
        ac::itemgrps::INDUSTRIAL_COMMAND_SHIP,
    ]
    .contains(&a_item.grp_id)
        || [
            ac::items::VENTURE,
            ac::items::VENTURE_CONSORTIUM_ISSUE,
            ac::items::PIONEER,
            ac::items::PIONEER_CONSORTIUM_ISSUE,
            ac::items::OUTRIDER,
        ]
        .contains(&a_item.id)
}
