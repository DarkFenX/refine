use crate::{
    ad,
    sol::{
        svc::Svc,
        uad::{Uad, item::Item},
    },
};

impl Svc {
    pub(in crate::sol) fn add_item(&mut self, uad: &Uad, item: &Item) {
        let is_a_item_loaded = item.is_loaded();
        self.notify_item_added(uad, item);
        if is_a_item_loaded {
            self.notify_item_loaded(uad, item)
        }
        self.switch_item_state(uad, item, ad::AState::Ghost, item.get_a_state());
    }
    pub(in crate::sol) fn remove_item(&mut self, uad: &Uad, item: &Item) {
        self.switch_item_state(uad, item, item.get_a_state(), ad::AState::Ghost);
        if item.is_loaded() {
            self.notify_item_unloaded(uad, item)
        }
        self.notify_item_removed(uad, item);
    }
    pub(in crate::sol) fn load_item(&mut self, uad: &Uad, item: &Item) {
        if item.is_loaded() {
            self.notify_item_loaded(uad, item);
            self.switch_item_state(uad, item, ad::AState::Ghost, item.get_a_state());
        }
    }
    pub(in crate::sol) fn unload_item(&mut self, uad: &Uad, item: &Item) {
        if item.is_loaded() {
            self.switch_item_state(uad, item, item.get_a_state(), ad::AState::Ghost);
            self.notify_item_unloaded(uad, item)
        }
    }
}
