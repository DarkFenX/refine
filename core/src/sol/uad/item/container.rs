use std::num::Wrapping;

use crate::{
    err::basic::ItemFoundError,
    sol::{ItemId, uad::item::Item},
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol) struct Items {
    counter: Wrapping<ItemId>,
    data: StMap<ItemId, Item>,
}
impl Items {
    pub(in crate::sol) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: StMap::new(),
        }
    }
    pub(in crate::sol) fn alloc_item_id(&mut self) -> ItemId {
        let start = self.counter;
        while self.data.contains_key(&self.counter.0) {
            self.counter += 1;
            if start == self.counter {
                panic!("ran out of item ID space");
            }
        }
        let item_id = self.counter.0;
        self.counter += 1;
        item_id
    }
    // Generic item methods
    pub(in crate::sol) fn add_item(&mut self, item: Item) {
        let item_id = item.get_item_id();
        self.data.insert(item_id, item);
    }
    pub(in crate::sol) fn get_item(&self, item_id: &ItemId) -> Result<&Item, ItemFoundError> {
        self.data
            .get(item_id)
            .ok_or_else(|| ItemFoundError { item_id: *item_id })
    }
    pub(in crate::sol) fn get_item_mut(&mut self, item_id: &ItemId) -> Result<&mut Item, ItemFoundError> {
        self.data
            .get_mut(item_id)
            .ok_or_else(|| ItemFoundError { item_id: *item_id })
    }
    pub(in crate::sol) fn remove_item(&mut self, item_id: &ItemId) -> Option<Item> {
        self.data.remove(item_id)
    }
    pub(in crate::sol) fn iter(&self) -> impl ExactSizeIterator<Item = &Item> {
        self.data.values()
    }
    pub(in crate::sol) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut Item> {
        self.data.values_mut()
    }
}
