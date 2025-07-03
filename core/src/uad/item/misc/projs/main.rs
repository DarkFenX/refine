use crate::{def::ItemKey, uad::ProjRange, util::RMap};

#[derive(Clone)]
pub(crate) struct Projs {
    pub(super) data: RMap<ItemKey, Option<ProjRange>>,
}
impl Projs {
    pub(in crate::uad::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    // Query methods
    pub(crate) fn get(&self, item_key: &ItemKey) -> Option<&Option<ProjRange>> {
        self.data.get(item_key)
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&ItemKey, &Option<ProjRange>)> {
        self.data.iter()
    }
    pub(crate) fn iter_projectee_item_keys(&self) -> impl ExactSizeIterator<Item = &ItemKey> {
        self.data.keys()
    }
    pub(crate) fn contains(&self, item_key: &ItemKey) -> bool {
        self.data.contains_key(item_key)
    }
    // Modification methods
    pub(crate) fn add(&mut self, item_key: ItemKey, range: Option<ProjRange>) {
        self.data.insert(item_key, range);
    }
    pub(crate) fn remove(&mut self, item_key: &ItemKey) -> Option<Option<ProjRange>> {
        self.data.remove(item_key)
    }
}
