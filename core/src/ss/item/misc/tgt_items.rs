use crate::{
    defs::{AttrVal, SsItemId},
    util::StMap,
};

pub(in crate::ss) struct TgtItems {
    data: StMap<SsItemId, Option<AttrVal>>,
}
impl TgtItems {
    pub(in crate::ss::item) fn new() -> Self {
        Self { data: StMap::new() }
    }
    pub(in crate::ss) fn add(&mut self, item_id: SsItemId, range: Option<AttrVal>) {
        self.data.insert(item_id, range);
    }
    pub(in crate::ss) fn remove(&mut self, item_id: &SsItemId) {
        self.data.remove(item_id);
    }
    pub(in crate::ss) fn iter(&self) -> impl ExactSizeIterator<Item = (&SsItemId, &Option<AttrVal>)> {
        self.data.iter()
    }
    pub(in crate::ss) fn iter_tgts(&self) -> impl ExactSizeIterator<Item = &SsItemId> {
        self.data.keys()
    }
    pub(in crate::ss) fn contains(&self, item_id: &SsItemId) -> bool {
        self.data.contains_key(item_id)
    }
}
