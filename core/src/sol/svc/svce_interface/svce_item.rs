use crate::sol::{
    svc::SolSvc,
    uad::{
        SolUad,
        item::{SolItem, SolItemState},
    },
};

impl SolSvc {
    pub(in crate::sol) fn add_item(&mut self, uad: &SolUad, item: &SolItem) {
        let is_a_item_loaded = item.is_loaded();
        self.notify_item_added(uad, item);
        if is_a_item_loaded {
            self.notify_item_loaded(uad, item)
        }
        self.switch_item_state(uad, item, SolItemState::Ghost, item.get_state());
    }
    pub(in crate::sol) fn remove_item(&mut self, uad: &SolUad, item: &SolItem) {
        self.switch_item_state(uad, item, item.get_state(), SolItemState::Ghost);
        if item.is_loaded() {
            self.notify_item_unloaded(uad, item)
        }
        self.notify_item_removed(uad, item);
    }
    pub(in crate::sol) fn load_item(&mut self, uad: &SolUad, item: &SolItem) {
        if item.is_loaded() {
            self.notify_item_loaded(uad, item);
            self.switch_item_state(uad, item, SolItemState::Ghost, item.get_state());
        }
    }
    pub(in crate::sol) fn unload_item(&mut self, uad: &SolUad, item: &SolItem) {
        if item.is_loaded() {
            self.switch_item_state(uad, item, item.get_state(), SolItemState::Ghost);
            self.notify_item_unloaded(uad, item)
        }
    }
}
