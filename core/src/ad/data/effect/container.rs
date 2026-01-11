use crate::{
    ad::{AEffect, AEffectId},
    util::RMap,
};

pub struct AEffects {
    pub(crate) data: RMap<AEffectId, AEffect>,
}
impl AEffects {
    pub fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub fn insert(&mut self, val: AEffect) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AEffect> {
        self.data.values()
    }
}
impl FromIterator<AEffect> for AEffects {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AEffect>,
    {
        Self {
            data: iter.into_iter().map(|v| (v.id, v)).collect(),
        }
    }
}
