use crate::ad::{AData, AItemListId};

pub(in crate::ad::generator::flow::s8_conv_post) fn fill_disallowed_in_wspace(a_data: &mut AData) {
    if let Some(a_item_list) = a_data.item_lists.data.get(&AItemListId::WORMHOLE_JUMP_BLACK_LIST) {
        for item_aid in a_item_list.item_ids.iter() {
            if let Some(a_item) = a_data.items.data.get_mut(item_aid) {
                a_item.disallowed_in_wspace = true;
            }
        }
    }
}
