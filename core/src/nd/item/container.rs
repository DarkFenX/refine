use std::sync::LazyLock;

use crate::{
    ad::AItemId,
    nd::item::{NItem, defs},
    util::RMap,
};

pub(crate) static N_ITEM_MAP: LazyLock<RMap<AItemId, NItem>> = LazyLock::new(get_item_map);

fn get_item_map() -> RMap<AItemId, NItem> {
    [defs::e35913_standup_conduit_generator::mk_n_item()]
        .into_iter()
        .map(|n_item| (n_item.aid, n_item))
        .collect()
}
