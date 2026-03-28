use crate::{
    rd::REffectId,
    svc::cycle::{CycleDataFull, CycleSeq},
    util::RMap,
};

pub(crate) struct CseqMap(RMap<REffectId, CycleSeq<CycleDataFull>>);
impl CseqMap {
    pub(crate) fn new() -> Self {
        Self(RMap::new())
    }
    pub(in crate::svc) fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }
    pub(in crate::svc) fn insert(
        &mut self,
        key: REffectId,
        value: CycleSeq<CycleDataFull>,
    ) -> Option<CycleSeq<CycleDataFull>> {
        self.0.insert(key, value)
    }
    pub(in crate::svc) fn get(&self, key: &REffectId) -> Option<&CycleSeq<CycleDataFull>> {
        self.0.get(key)
    }
    pub(in crate::svc) fn iter(&self) -> impl ExactSizeIterator<Item = (&REffectId, &CycleSeq<CycleDataFull>)> {
        self.0.iter()
    }
    pub(in crate::svc) fn retain(&mut self, func: impl FnMut(&REffectId, &mut CycleSeq<CycleDataFull>) -> bool) {
        self.0.retain(func)
    }
    pub(in crate::svc) fn remove(&mut self, key: &REffectId) -> Option<CycleSeq<CycleDataFull>> {
        self.0.remove(key)
    }
    pub(in crate::svc) fn clear(&mut self) {
        self.0.clear()
    }
}
