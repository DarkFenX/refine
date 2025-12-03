use std::sync::LazyLock;

use crate::nd::item_list::{NItemList, data};

pub(crate) static N_ITEM_LISTS: LazyLock<Vec<NItemList>> = LazyLock::new(get_item_lists);

fn get_item_lists() -> Vec<NItemList> {
    vec![
        data::c1_ships::mk_n_item_list(),
        data::c2_ships_drones_fighters_entities::mk_n_item_list(),
        data::c3_capitals_freighters::mk_n_item_list(),
        data::c4_panic_eligible::mk_n_item_list(),
    ]
}
