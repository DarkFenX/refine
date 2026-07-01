use crate::ad::{ADataGenerator, AItemListId};

impl ADataGenerator {
    pub(super) fn fill_disallowed_in_wspace(&mut self) {
        if let Some(a_item_list) = self.a_data.item_lists.data.get(&AItemListId::WORMHOLE_JUMP_BLACK_LIST) {
            for item_aid in a_item_list.item_ids.iter() {
                if let Some(a_item) = self.a_data.items.data.get_mut(item_aid) {
                    a_item.disallowed_in_wspace = true;
                }
            }
        }
    }
}
