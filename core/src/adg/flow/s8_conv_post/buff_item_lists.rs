// Buffs specify what they can affect via item lists. For efficiency of attribute calculation,
// information about item lists used by buffs stored on items belonging to those item lists.

use crate::{ad::AData, util::RSet};

pub(in crate::adg::flow::s8_conv_post) fn fill_buff_item_lists(a_data: &mut AData) {
    // Collect item lists which are used in buffs
    let mut involved_item_list_ids = RSet::new();
    for a_effect in a_data.effects.values() {
        if let Some(a_buff_info) = &a_effect.buff_info {
            involved_item_list_ids.extend(a_buff_info.iter_a_item_list_ids());
        }
    }
    // Put data about buff-involved item lists onto items which belong to those lists
    for a_item_list_id in involved_item_list_ids {
        if let Some(a_item_list) = a_data.item_lists.get(&a_item_list_id) {
            for a_item_id in a_item_list.item_ids.iter() {
                if let Some(a_item) = a_data.items.get_mut(a_item_id) {
                    a_item.buff_item_list_ids.push(a_item_list_id);
                }
            }
        }
    }
}
