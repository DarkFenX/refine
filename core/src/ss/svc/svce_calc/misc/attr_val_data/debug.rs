use crate::ss::SsView;

use super::AttrValData;

impl AttrValData {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> bool {
        for (item_id, attr_map) in self.data.iter() {
            // All items are supposed to have adapted item available
            let item = match ss_view.items.get_item(item_id) {
                Ok(item) => item,
                _ => return false,
            };
            if item.get_a_item().is_err() {
                return false;
            }
            // All attributes are supposed to be available too
            for attr_id in attr_map.keys() {
                if ss_view.src.get_a_attr(attr_id).is_none() {
                    return false;
                }
            }
        }
        true
    }
}
