use crate::{
    ad::{AAbil, AAbilId},
    util::RMap,
};

pub struct AAbils {
    pub(crate) data: RMap<AAbilId, AAbil>,
}
impl AAbils {
    pub fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub fn insert(&mut self, val: AAbil) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AAbil> {
        self.data.values()
    }
}
impl FromIterator<AAbil> for AAbils {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AAbil>,
    {
        Self {
            data: iter.into_iter().map(|v| (v.id, v)).collect(),
        }
    }
}
