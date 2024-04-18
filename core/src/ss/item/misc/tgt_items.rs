use crate::{defs::SsItemId, util::StSet};

pub(in crate::ss) struct TgtItems {
    data: StSet<SsItemId>,
}
impl TgtItems {
    pub(in crate::ss::item) fn new() -> Self {
        Self { data: StSet::new() }
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
