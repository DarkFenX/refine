use crate::{
    ad::{AItemList, AItemListId},
    util::RMap,
};

pub struct AItemLists {
    pub(crate) data: RMap<AItemListId, AItemList>,
}
impl AItemLists {
    pub fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub fn insert(&mut self, val: AItemList) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AItemList> {
        self.data.values()
    }
}
impl FromIterator<AItemList> for AItemLists {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AItemList>,
    {
        Self {
            data: iter.into_iter().map(|v| (v.id, v)).collect(),
        }
    }
}
