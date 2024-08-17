use crate::sol::{
    item::{SolItem, SolItemState},
    svc::SolSvcs,
    SolView,
};

impl SolSvcs {
    pub(in crate::sol) fn add_item(&mut self, sol_view: &SolView, item: &SolItem) {
        let is_a_item_loaded = item.is_loaded();
        self.notify_item_added(sol_view, item);
        if is_a_item_loaded {
            self.notify_item_loaded(sol_view, item)
        }
        self.switch_item_state(sol_view, item, SolItemState::Ghost, item.get_state());
    }
    pub(in crate::sol) fn remove_item(&mut self, sol_view: &SolView, item: &SolItem) {
        self.switch_item_state(sol_view, item, item.get_state(), SolItemState::Ghost);
        if item.is_loaded() {
            self.notify_item_unloaded(sol_view, item)
        }
        self.notify_item_removed(sol_view, item);
    }
    pub(in crate::sol) fn load_item(&mut self, sol_view: &SolView, item: &SolItem) {
        self.notify_item_loaded(sol_view, item);
        self.switch_item_state(sol_view, item, SolItemState::Ghost, item.get_state());
    }
    pub(in crate::sol) fn unload_item(&mut self, sol_view: &SolView, item: &SolItem) {
        self.switch_item_state(sol_view, item, item.get_state(), SolItemState::Ghost);
        self.notify_item_unloaded(sol_view, item)
    }
}
