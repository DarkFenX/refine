use crate::ad::AEffectId;

pub struct AEffectStopIds {
    data: Vec<AEffectId>,
}
impl AEffectStopIds {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn insert(&mut self, val: AEffectId) {
        self.data.push(val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AEffectId> {
        self.data.iter()
    }
}
impl FromIterator<AEffectId> for AEffectStopIds {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AEffectId>,
    {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AEffectStopIds {
    pub(crate) fn extend<I: IntoIterator<Item = AEffectId>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
    pub(in crate::ad) fn contains(&self, val: &AEffectId) -> bool {
        self.data.contains(val)
    }
}
