use crate::{
    ad,
    sol::{
        ItemKey,
        svc::Svc,
        uad::{Uad, item::UadItem},
    },
};

impl Svc {
    pub(in crate::sol) fn add_item(&mut self, uad: &Uad, item_key: ItemKey, item: &UadItem) {
        self.notify_item_added(uad, item_key, item);
        if item.is_loaded() {
            self.notify_item_loaded(uad, item_key, item)
        }
        self.switch_item_state(uad, item_key, item, ad::AState::Ghost, item.get_a_state());
    }
    pub(in crate::sol) fn remove_item(&mut self, uad: &Uad, item_key: ItemKey, item: &UadItem) {
        self.switch_item_state(uad, item_key, item, item.get_a_state(), ad::AState::Ghost);
        if item.is_loaded() {
            self.notify_item_unloaded(uad, item_key, item)
        }
        self.notify_item_removed(uad, item_key, item);
    }
    pub(in crate::sol) fn load_item(&mut self, uad: &Uad, item_key: ItemKey, item: &UadItem) {
        if item.is_loaded() {
            self.notify_item_loaded(uad, item_key, item);
            self.switch_item_state(uad, item_key, item, ad::AState::Ghost, item.get_a_state());
        }
    }
    pub(in crate::sol) fn unload_item(&mut self, uad: &Uad, item_key: ItemKey, item: &UadItem) {
        if item.is_loaded() {
            self.switch_item_state(uad, item_key, item, item.get_a_state(), ad::AState::Ghost);
            self.notify_item_unloaded(uad, item_key, item)
        }
    }
}
