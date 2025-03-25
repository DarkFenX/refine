use crate::{
    sol::{AttrVal, ItemId},
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol) struct Projs {
    data: StMap<ItemId, Option<AttrVal>>,
}
impl Projs {
    pub(in crate::sol::uad::item) fn new() -> Self {
        Self { data: StMap::new() }
    }
    pub(in crate::sol) fn add(&mut self, item_id: ItemId, range: Option<AttrVal>) {
        self.data.insert(item_id, range);
    }
    pub(in crate::sol) fn remove(&mut self, item_id: &ItemId) -> Option<Option<AttrVal>> {
        self.data.remove(item_id)
    }
    pub(in crate::sol) fn get(&self, item_id: &ItemId) -> Option<&Option<AttrVal>> {
        self.data.get(item_id)
    }
    pub(in crate::sol) fn iter(&self) -> impl ExactSizeIterator<Item = (&ItemId, &Option<AttrVal>)> {
        self.data.iter()
    }
    pub(in crate::sol) fn iter_items(&self) -> impl ExactSizeIterator<Item = &ItemId> {
        self.data.keys()
    }
    pub(in crate::sol) fn contains(&self, item_id: &ItemId) -> bool {
        self.data.contains_key(item_id)
    }
}
