use crate::sol::{
    svc::SolSvc,
    uad::{
        item::{SolItem, SolItemState},
        SolUad,
    },
};

impl SolSvc {
    pub(in crate::sol) fn switch_item_state(
        &mut self,
        uad: &SolUad,
        item: &SolItem,
        old_item_state: SolItemState,
        new_item_state: SolItemState,
    ) {
        if new_item_state > old_item_state {
            for state in SolItemState::iter().filter(|v| **v > old_item_state && **v <= new_item_state) {
                self.notify_state_activated(uad, item, state);
                if item.is_loaded() {
                    self.notify_item_state_activated_loaded(uad, item, state);
                }
            }
        } else if new_item_state < old_item_state {
            for state in SolItemState::iter().filter(|v| **v > new_item_state && **v <= old_item_state) {
                if item.is_loaded() {
                    self.notify_item_state_deactivated_loaded(uad, item, state);
                }
                self.notify_state_deactivated(uad, item, state);
            }
        }
        self.process_effects(uad, item, new_item_state);
    }
}
