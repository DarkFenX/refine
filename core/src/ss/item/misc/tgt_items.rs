use std::collections::HashSet;

use crate::defs::SsItemId;

pub(in crate::ss) struct TgtItems {
    data: HashSet<SsItemId>,
}
impl TgtItems {
    pub(in crate::ss::item) fn new() -> Self {
        Self { data: HashSet::new() }
    }
    pub(in crate::ss) fn add(&mut self, item_id: SsItemId) {
        self.data.insert(item_id);
    }
    pub(in crate::ss) fn remove(&mut self, item_id: &SsItemId) {
        self.data.remove(item_id);
    }
    pub(in crate::ss) fn iter(&self) -> impl ExactSizeIterator<Item = &SsItemId> {
        self.data.iter()
    }
    pub(in crate::ss) fn contains(&self, item_id: &SsItemId) -> bool {
        self.data.contains(item_id)
    }
}
