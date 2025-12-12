use std::collections::hash_map::Entry;

use crate::{
    ad::{AData, AItemList},
    nd::N_ITEM_LIST_MAP,
    util::RSet,
};

pub(in crate::ad::generator::flow::s7_custom) fn customize_item_lists(a_data: &mut AData) {
    for n_item_list in N_ITEM_LIST_MAP.values() {
        if let Some(item_filter) = n_item_list.adg_item_filter_fn {
            let a_item_list = match a_data.item_lists.entry(n_item_list.aid) {
                Entry::Occupied(entry) => {
                    let a_item_list = entry.into_mut();
                    if !a_item_list.item_ids.is_empty() {
                        tracing::info!(
                            "item list {}: clearing to overwrite with custom contents",
                            a_item_list.id
                        );
                        a_item_list.item_ids.clear();
                    }
                    a_item_list
                }
                Entry::Vacant(entry) => entry.insert(AItemList {
                    id: n_item_list.aid,
                    item_ids: RSet::new(),
                }),
            };
            for a_item in a_data.items.values() {
                if item_filter(a_item) {
                    a_item_list.item_ids.insert(a_item.id);
                }
            }
        }
    }
}
