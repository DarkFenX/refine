use std::sync::LazyLock;

use crate::{
    ad::AItemListId,
    nd::item_list::{NItemList, defs},
    util::RMap,
};

pub(crate) static N_ITEM_LIST_MAP: LazyLock<RMap<AItemListId, NItemList>> = LazyLock::new(get_item_list_map);

fn get_item_list_map() -> RMap<AItemListId, NItemList> {
    [
        defs::c1_ships::mk_n_item_list(),
        defs::c2_ships_drones_fighters::mk_n_item_list(),
        defs::c3_ships_drones_fighters_entities::mk_n_item_list(),
        defs::c4_capitals_freighters::mk_n_item_list(),
        defs::c5_panic_eligible::mk_n_item_list(),
    ]
    .into_iter()
    .map(|n_item_list| (n_item_list.aid, n_item_list))
    .collect()
}
