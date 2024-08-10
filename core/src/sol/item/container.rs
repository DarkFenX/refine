use std::num::Wrapping;

use crate::{
    defs::{SolFitId, SolItemId},
    sol::{
        err::basic::{ItemAllocError, ItemFoundError},
        item::SolItem,
    },
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol) struct SolItems {
    counter: Wrapping<SolItemId>,
    data: StMap<SolItemId, SolItem>,
}
impl SolItems {
    pub(in crate::sol) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: StMap::new(),
        }
    }
    pub(in crate::sol) fn alloc_item_id(&mut self) -> Result<SolItemId, ItemAllocError> {
        let start = self.counter;
        while self.data.contains_key(&self.counter.0) {
            self.counter += 1;
            if start == self.counter {
                return Err(ItemAllocError::new());
            }
        }
        let item_id = self.counter.0;
        self.counter += 1;
        Ok(item_id)
    }
    // Generic item methods
    pub(in crate::sol) fn add_item(&mut self, item: SolItem) {
        let item_id = item.get_id();
        self.data.insert(item_id, item);
    }
    pub(in crate::sol) fn get_item(&self, item_id: &SolItemId) -> Result<&SolItem, ItemFoundError> {
        self.data.get(item_id).ok_or_else(|| ItemFoundError::new(*item_id))
    }
    pub(in crate::sol) fn get_item_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolItem, ItemFoundError> {
        self.data.get_mut(item_id).ok_or_else(|| ItemFoundError::new(*item_id))
    }
    pub(in crate::sol) fn remove_item(&mut self, item_id: &SolItemId) -> Option<SolItem> {
        self.data.remove(item_id)
    }
    pub(in crate::sol) fn remove_fit_items(&mut self, fit_id: &SolFitId) {
        self.data.retain(|_, v| v.get_fit_id() != Some(*fit_id));
    }

    pub(in crate::sol) fn iter(&self) -> impl ExactSizeIterator<Item = &SolItem> {
        self.data.values()
    }
    pub(in crate::sol) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut SolItem> {
        self.data.values_mut()
    }
}
