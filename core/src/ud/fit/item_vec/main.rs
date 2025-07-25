use crate::{def::Idx, ud::UItemKey};

#[derive(Clone)]
pub(crate) struct UItemVec {
    pub(super) data: Vec<Option<UItemKey>>,
    pub(super) item_count: usize,
}
impl UItemVec {
    pub(in crate::ud::fit) fn new() -> Self {
        Self {
            data: Vec::new(),
            item_count: 0,
        }
    }
    pub(crate) fn iter_all(&self) -> impl ExactSizeIterator<Item = &Option<UItemKey>> {
        self.data.iter()
    }
    pub(crate) fn iter_keys(&self) -> impl Iterator<Item = &UItemKey> {
        self.data.iter().filter_map(|v| v.as_ref())
    }
    pub(crate) fn iter_keys_from(&self, start: Idx) -> impl Iterator<Item = &UItemKey> {
        let start = Idx::min(start, self.data.len());
        self.data[start..].iter().filter_map(|v| v.as_ref())
    }
    pub(crate) fn get(&self, index: Idx) -> Option<UItemKey> {
        if index >= self.data.len() {
            return None;
        }
        self.data[index]
    }
    pub(crate) fn len(&self) -> Idx {
        self.data.len()
    }
    pub(crate) fn item_count(&self) -> Idx {
        self.item_count
    }
    pub(crate) fn inner(&self) -> &Vec<Option<UItemKey>> {
        &self.data
    }
    // Modification methods
    pub(crate) fn append(&mut self, val: UItemKey) -> Idx {
        let pos = self.data.len();
        self.data.push(Some(val));
        self.item_count += 1;
        pos
    }
    pub(crate) fn equip(&mut self, val: UItemKey) -> Idx {
        for (i, item) in self.data.iter().enumerate() {
            if item.is_none() {
                let _ = self.data[i].replace(val);
                self.item_count += 1;
                return i;
            }
        }
        self.append(val)
    }
    pub(crate) fn insert(&mut self, index: Idx, val: UItemKey) -> bool {
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
    pub(crate) fn place(&mut self, index: Idx, val: UItemKey) {
        if index >= self.data.len() {
            // Resize with extra element to be able to replace it
            self.data.resize(index + 1, None);
        }
        if self.data[index].replace(val).is_none() {
            self.item_count += 1;
        }
    }
    pub(crate) fn free(&mut self, &val: &UItemKey) {
        if let Some(pos) = self.data.iter().position(|&v| v == Some(val)) {
            self.data[pos] = None;
            self.item_count -= 1;
            if pos + 1 == self.data.len() {
                self.clear_tail();
            }
        }
    }
    pub(crate) fn remove(&mut self, &val: &UItemKey) -> Option<Idx> {
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
