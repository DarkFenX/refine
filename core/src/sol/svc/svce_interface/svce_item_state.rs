use crate::{
    ad,
    sol::{
        ItemKey,
        svc::Svc,
        uad::{Uad, item::Item},
    },
};

impl Svc {
    pub(in crate::sol) fn switch_item_state(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
        item: &Item,
        old_item_a_state: ad::AState,
        new_item_a_state: ad::AState,
    ) {
        if new_item_a_state > old_item_a_state {
            for a_state in ad::AState::iter().filter(|v| **v > old_item_a_state && **v <= new_item_a_state) {
                self.notify_state_activated(item_key, item, a_state);
                if item.is_loaded() {
                    self.notify_item_state_activated_loaded(item_key, item, a_state);
                }
            }
        } else if new_item_a_state < old_item_a_state {
            for a_state in ad::AState::iter()
                .rev()
                .filter(|v| **v > new_item_a_state && **v <= old_item_a_state)
            {
                if item.is_loaded() {
                    self.notify_item_state_deactivated_loaded(&item_key, item, a_state);
                }
                self.notify_state_deactivated(&item_key, item, a_state);
            }
        }
        self.process_effects(uad, item_key, item, new_item_a_state);
    }
}
