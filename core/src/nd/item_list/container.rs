use std::sync::LazyLock;

use crate::{
    ad::AItemListId,
    nd::item_list::{NItemList, data},
    util::RMap,
};

pub(crate) static N_ITEM_LIST_MAP: LazyLock<RMap<AItemListId, NItemList>> = LazyLock::new(get_item_list_map);

fn get_item_list_map() -> RMap<AItemListId, NItemList> {
    [
        data::c1_ships::mk_n_item_list(),
        data::c2_ships_drones_fighters_entities::mk_n_item_list(),
        data::c3_capitals_freighters::mk_n_item_list(),
        data::c4_panic_eligible::mk_n_item_list(),
    ]
    .into_iter()
    .map(|v| (v.aid, v))
    .collect()
}
