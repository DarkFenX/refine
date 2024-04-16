use crate::ss::SsView;

use super::RunningEffects;

impl RunningEffects {
    pub(in crate::ss::svc) fn debug_consistency_check(&self, ss_view: &SsView) -> bool {
        for (item_id, effect_ids) in self.data.iter() {
            // All items are supposed to have adapted item available
            let item = match ss_view.items.get_item(item_id) {
                Ok(item) => item,
                _ => return false,
            };
            if item.get_a_item().is_err() {
                return false;
            }
            // All effects which are running are supposed to be available in data source
            for effect_id in effect_ids.iter() {
                if ss_view.src.get_a_effect(effect_id).is_none() {
                    return false;
                }
            }
        }
        true
    }
}
