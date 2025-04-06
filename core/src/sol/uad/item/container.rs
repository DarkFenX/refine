use std::num::Wrapping;

use slab::Slab;

use crate::{
    err::basic::ItemFoundError,
    sol::{ItemId, ItemKey, uad::item::Item},
    util::RMap,
};

#[derive(Clone)]
pub(in crate::sol) struct Items {
    counter: Wrapping<ItemId>,
    data: Slab<Item>,
    id_to_key: RMap<ItemId, ItemKey>,
}
impl Items {
    pub(in crate::sol) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: Slab::with_capacity(1000),
            id_to_key: RMap::with_capacity(1000),
        }
    }
    pub(in crate::sol) fn alloc_item_id(&mut self) -> ItemId {
        let start = self.counter;
        while self.id_to_key.contains_key(&self.counter.0) {
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
    pub(in crate::sol) fn add(&mut self, item: Item) -> ItemKey {
        let item_id = item.get_item_id();
        let item_key = self.data.insert(item);
        self.id_to_key.insert(item_id, item_key);
        item_key
    }
    pub(in crate::sol) fn get_by_key(&self, item_key: ItemKey) -> Option<&Item> {
        self.data.get(item_key)
    }
    pub(in crate::sol) fn get_by_id(&self, item_id: &ItemId) -> Result<&Item, ItemFoundError> {
        let item_key = self
            .id_to_key
            .get(item_id)
            .ok_or(ItemFoundError { item_id: *item_id })?;
        self.get_by_key(*item_key).ok_or(ItemFoundError { item_id: *item_id })
    }
    pub(in crate::sol) fn get_mut_by_key(&mut self, item_key: ItemKey) -> Option<&mut Item> {
        self.data.get_mut(item_key)
    }
    pub(in crate::sol) fn get_mut_by_id(&mut self, item_id: &ItemId) -> Result<&mut Item, ItemFoundError> {
        let item_key = self
            .id_to_key
            .get(item_id)
            .ok_or(ItemFoundError { item_id: *item_id })?;
        self.get_mut_by_key(*item_key)
            .ok_or(ItemFoundError { item_id: *item_id })
    }
    pub(in crate::sol) fn remove_by_key(&mut self, item_key: ItemKey) -> Option<Item> {
        let item = self.data.try_remove(item_key)?;
        self.id_to_key.remove(&item.get_item_id());
        Some(item)
    }
    pub(in crate::sol) fn remove_by_id(&mut self, item_id: &ItemId) -> Option<Item> {
        let item_key = self.id_to_key.remove(item_id)?;
        self.data.try_remove(item_key)
    }
    pub(in crate::sol) fn iter(&self) -> impl ExactSizeIterator<Item = &Item> {
        self.data.iter().map(|(key, item)| item)
    }
    pub(in crate::sol) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut Item> {
        self.data.iter_mut().map(|(key, item)| item)
    }
}
