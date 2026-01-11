use crate::{
    ad::{AAttr, AAttrId},
    util::RMap,
};

pub struct AAttrs {
    pub(crate) data: RMap<AAttrId, AAttr>,
}
impl AAttrs {
    pub fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub fn insert(&mut self, val: AAttr) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AAttr> {
        self.data.values()
    }
}
impl FromIterator<AAttr> for AAttrs {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AAttr>,
    {
        Self {
            data: iter.into_iter().map(|v| (v.id, v)).collect(),
        }
    }
}
