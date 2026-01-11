use crate::{
    ad::{AItem, AItemId},
    util::RMap,
};

pub struct AItems {
    pub(crate) data: RMap<AItemId, AItem>,
}
impl AItems {
    pub fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub fn insert(&mut self, val: AItem) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AItem> {
        self.data.values()
    }
}
impl FromIterator<AItem> for AItems {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AItem>,
    {
        Self {
            data: iter.into_iter().map(|v| (v.id, v)).collect(),
        }
    }
}
