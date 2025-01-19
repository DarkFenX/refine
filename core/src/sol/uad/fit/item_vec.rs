use crate::defs::{Idx, SolItemId};

#[derive(Clone)]
pub(in crate::sol) struct SolItemVec {
    data: Vec<Option<SolItemId>>,
}
impl SolItemVec {
    pub(in crate::sol) fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub(in crate::sol) fn iter_all(&self) -> impl ExactSizeIterator<Item = &Option<SolItemId>> {
        self.data.iter()
    }
    pub(in crate::sol) fn iter_ids(&self) -> impl Iterator<Item = &SolItemId> {
        self.data.iter().filter_map(|v| v.as_ref())
    }

    pub(in crate::sol) fn get(&self, index: Idx) -> Option<SolItemId> {
        if index >= self.data.len() {
            return None;
        }
        self.data[index]
    }
    pub(in crate::sol) fn len(&self) -> Idx {
        self.data.len()
    }
    pub(in crate::sol) fn inner(&self) -> &Vec<Option<SolItemId>> {
        &self.data
    }
    // Modification methods
    pub(in crate::sol) fn append(&mut self, val: SolItemId) -> Idx {
        let pos = self.data.len();
        self.data.push(Some(val));
        pos
    }
    pub(in crate::sol) fn equip(&mut self, val: SolItemId) -> Idx {
        for (i, item) in self.data.iter().enumerate() {
            if item.is_none() {
                let _ = std::mem::replace(&mut self.data[i], Some(val));
                return i;
            }
        }
        self.append(val)
    }
    pub(in crate::sol) fn insert(&mut self, index: Idx, val: SolItemId) -> bool {
        if index > self.data.len() {
            // Resize with extra element & replace it, to avoid extra allocations
            self.data.resize(index + 1, None);
            let _ = std::mem::replace(&mut self.data[index], Some(val));
            false
        } else {
            self.data.insert(index, Some(val));
            true
        }
    }
    pub(in crate::sol) fn place(&mut self, index: Idx, val: SolItemId) {
        if index >= self.data.len() {
            // Resize with extra element to be able to replace it
            self.data.resize(index + 1, None);
        }
        let _ = std::mem::replace(&mut self.data[index], Some(val));
    }

    pub(in crate::sol) fn free(&mut self, &val: &SolItemId) {
        if let Some(pos) = self.data.iter().position(|&v| v == Some(val)) {
            self.data[pos] = None;
            if pos + 1 == self.data.len() {
                while self.data.last().map_or(false, |last| last.is_none()) {
                    self.data.pop();
                }
            }
        }
    }
}
