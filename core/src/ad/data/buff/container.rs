use crate::{
    ad::{ABuff, ABuffId},
    util::RMap,
};

pub struct ABuffs {
    pub(crate) data: RMap<ABuffId, ABuff>,
}
impl ABuffs {
    pub fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub fn insert(&mut self, val: ABuff) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &ABuff> {
        self.data.values()
    }
}
impl FromIterator<ABuff> for ABuffs {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = ABuff>,
    {
        Self {
            data: iter.into_iter().map(|v| (v.id, v)).collect(),
        }
    }
}
