use crate::{
    sol::{AttrVal, ItemKey},
    util::RMap,
};

#[derive(Clone)]
pub(in crate::sol) struct Projs {
    data: RMap<ItemKey, Option<AttrVal>>,
}
impl Projs {
    pub(in crate::sol::uad::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    // Query methods
    pub(in crate::sol) fn get(&self, item_key: &ItemKey) -> Option<&Option<AttrVal>> {
        self.data.get(item_key)
    }
    pub(in crate::sol) fn iter(&self) -> impl ExactSizeIterator<Item = (&ItemKey, &Option<AttrVal>)> {
        self.data.iter()
    }
    pub(in crate::sol) fn iter_projectee_item_keys(&self) -> impl ExactSizeIterator<Item = &ItemKey> {
        self.data.keys()
    }
    pub(in crate::sol) fn contains(&self, item_key: &ItemKey) -> bool {
        self.data.contains_key(item_key)
    }
    // Modification methods
    pub(in crate::sol) fn add(&mut self, item_key: ItemKey, range: Option<AttrVal>) {
        self.data.insert(item_key, range);
    }
    pub(in crate::sol) fn remove(&mut self, item_key: &ItemKey) -> Option<Option<AttrVal>> {
        self.data.remove(item_key)
    }
}
