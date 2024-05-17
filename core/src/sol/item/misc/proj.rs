use crate::{
    defs::{AttrVal, SolItemId},
    util::StMap,
};

pub(in crate::sol) struct SolProjs {
    data: StMap<SolItemId, Option<AttrVal>>,
}
impl SolProjs {
    pub(in crate::sol::item) fn new() -> Self {
        Self { data: StMap::new() }
    }
    pub(in crate::sol) fn add(&mut self, item_id: SolItemId, range: Option<AttrVal>) {
        self.data.insert(item_id, range);
    }
    pub(in crate::sol) fn remove(&mut self, item_id: &SolItemId) -> Option<Option<AttrVal>> {
        self.data.remove(item_id)
    }
    pub(in crate::sol) fn get(&self, item_id: &SolItemId) -> Option<&Option<AttrVal>> {
        self.data.get(item_id)
    }
    pub(in crate::sol) fn iter(&self) -> impl ExactSizeIterator<Item = (&SolItemId, &Option<AttrVal>)> {
        self.data.iter()
    }
    pub(in crate::sol) fn iter_items(&self) -> impl ExactSizeIterator<Item = &SolItemId> {
        self.data.keys()
    }
    pub(in crate::sol) fn contains(&self, item_id: &SolItemId) -> bool {
        self.data.contains_key(item_id)
    }
}
