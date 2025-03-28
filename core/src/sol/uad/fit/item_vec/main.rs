use crate::sol::{Idx, ItemId};

#[derive(Clone)]
pub(in crate::sol) struct ItemVec {
    pub(super) data: Vec<Option<ItemId>>,
    pub(super) item_count: usize,
}
impl ItemVec {
    pub(in crate::sol) fn new() -> Self {
        Self {
            data: Vec::new(),
            item_count: 0,
        }
    }
    pub(in crate::sol) fn iter_all(&self) -> impl ExactSizeIterator<Item = &Option<ItemId>> {
        self.data.iter()
    }
    pub(in crate::sol) fn iter_ids(&self) -> impl Iterator<Item = &ItemId> {
        self.data.iter().filter_map(|v| v.as_ref())
    }
    pub(in crate::sol) fn iter_ids_from(&self, start: Idx) -> impl Iterator<Item = &ItemId> {
        let start = Idx::min(start, self.data.len());
        self.data[start..].iter().filter_map(|v| v.as_ref())
    }
    pub(in crate::sol) fn get(&self, index: Idx) -> Option<ItemId> {
        if index >= self.data.len() {
            return None;
        }
        self.data[index]
    }
    pub(in crate::sol) fn len(&self) -> Idx {
        self.data.len()
    }
    pub(in crate::sol) fn item_count(&self) -> Idx {
        self.item_count
    }
    pub(in crate::sol) fn inner(&self) -> &Vec<Option<ItemId>> {
        &self.data
    }
    // Modification methods
    pub(in crate::sol) fn append(&mut self, val: ItemId) -> Idx {
        let pos = self.data.len();
        self.data.push(Some(val));
        self.item_count += 1;
        pos
    }
    pub(in crate::sol) fn equip(&mut self, val: ItemId) -> Idx {
        for (i, item) in self.data.iter().enumerate() {
            if item.is_none() {
                let _ = self.data[i].replace(val);
                self.item_count += 1;
                return i;
            }
        }
        self.append(val)
    }
    pub(in crate::sol) fn insert(&mut self, index: Idx, val: ItemId) -> bool {
        // Returns true if other modules might need their positions updated
        if index > self.data.len() {
            // Resize with extra element & replace it, to avoid extra allocations
            self.data.resize(index + 1, None);
            let _ = self.data[index].replace(val);
            self.item_count += 1;
            false
        } else {
            self.data.insert(index, Some(val));
            self.item_count += 1;
            true
        }
    }
    pub(in crate::sol) fn place(&mut self, index: Idx, val: ItemId) {
        if index >= self.data.len() {
            // Resize with extra element to be able to replace it
            self.data.resize(index + 1, None);
        }
        if self.data[index].replace(val).is_none() {
            self.item_count += 1;
        }
    }
    pub(in crate::sol) fn free(&mut self, &val: &ItemId) {
        if let Some(pos) = self.data.iter().position(|&v| v == Some(val)) {
            self.data[pos] = None;
            self.item_count -= 1;
            if pos + 1 == self.data.len() {
                self.clear_tail();
            }
        }
    }
    pub(in crate::sol) fn remove(&mut self, &val: &ItemId) -> Option<Idx> {
        // Returns Some if other modules might need their positions updated
        if let Some(pos) = self.data.iter().position(|&v| v == Some(val)) {
            self.data.remove(pos);
            self.item_count -= 1;
            if pos == self.data.len() {
                self.clear_tail();
                return None;
            }
            return Some(pos);
        }
        None
    }
    // Private methods
    fn clear_tail(&mut self) {
        while self.data.last().is_some_and(|last| last.is_none()) {
            self.data.pop();
        }
    }
}
