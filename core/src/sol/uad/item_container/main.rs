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
    pub(super) data: Slab<Item>,
    pub(super) id_to_key: RMap<ItemId, ItemKey>,
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
    pub(in crate::sol) fn add(&mut self, item: Item) -> ItemKey {
        let item_id = item.get_item_id();
        let item_key = self.data.insert(item);
        self.id_to_key.insert(item_id, item_key);
        item_key
    }
    pub(in crate::sol) fn key_by_id(&self, item_id: &ItemId) -> Option<ItemKey> {
        self.id_to_key.get(item_id).copied()
    }
    pub(in crate::sol) fn key_by_id_err(&self, item_id: &ItemId) -> Result<ItemKey, ItemFoundError> {
        match self.id_to_key.get(item_id) {
            Some(item_key) => Ok(*item_key),
            None => Err(ItemFoundError { item_id: *item_id }),
        }
    }
    pub(in crate::sol) fn id_by_key(&self, item_key: ItemKey) -> ItemId {
        self.get(item_key).get_item_id()
    }
    pub(in crate::sol) fn try_get(&self, item_key: ItemKey) -> Option<&Item> {
        self.data.get(item_key)
    }
    pub(in crate::sol) fn get(&self, item_key: ItemKey) -> &Item {
        // Keys are supposed to be valid throughout whole lib, so just unwrap
        self.data.get(item_key).unwrap()
    }
    pub(in crate::sol) fn get_mut(&mut self, item_key: ItemKey) -> &mut Item {
        // Keys are supposed to be valid throughout whole lib, so just unwrap
        self.data.get_mut(item_key).unwrap()
    }
    pub(in crate::sol) fn remove(&mut self, item_key: ItemKey) -> Item {
        // Keys are supposed to be valid throughout whole lib, so use non-try removal
        let item = self.data.remove(item_key);
        self.id_to_key.remove(&item.get_item_id());
        item
    }
    pub(in crate::sol) fn iter(&self) -> impl ExactSizeIterator<Item = (ItemKey, &Item)> {
        self.data.iter()
    }
    pub(in crate::sol) fn keys(&self) -> impl ExactSizeIterator<Item = ItemKey> {
        self.data.iter().map(|(key, _)| key)
    }
    pub(in crate::sol) fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut Item> {
        self.data.iter_mut().map(|(_, item)| item)
    }
}
