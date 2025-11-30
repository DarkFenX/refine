use crate::{ac, ad::AData};

pub(in crate::adg::flow::conv_post) fn fill_disallowed_in_wspace(a_data: &mut AData) {
    if let Some(a_item_list) = a_data.item_lists.get(&ac::itemlists::WORMHOLE_JUMP_BLACK_LIST) {
        for a_item_id in a_item_list.item_ids.iter() {
            if let Some(a_item) = a_data.items.get_mut(a_item_id) {
                a_item.disallowed_in_wspace = true;
            }
        }
    }
}
