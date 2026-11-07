use crate::{
    rd::REffectId,
    svc::cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq},
};

pub(super) type CseqFull = CycleSeq<CycleDataFull, CSeqHardDtFull>;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Access interface
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Cycle sequences of an item's effects.
pub(in crate::svc) struct CseqMap {
    // Items have few cycling effects (1-5), so entries are kept in a vector and searched linearly.
    entries: Vec<(REffectId, CseqFull)>,
}
impl CseqMap {
    pub(in crate::svc) fn get(&self, effect_rid: &REffectId) -> Option<&CseqFull> {
        self.entries
            .iter()
            .find(|(entry_effect_rid, _)| entry_effect_rid == effect_rid)
            .map(|(_, cseq)| cseq)
    }
    pub(in crate::svc) fn iter(&self) -> impl ExactSizeIterator<Item = (&REffectId, &CseqFull)> {
        self.entries.iter().map(|(effect_rid, cseq)| (effect_rid, cseq))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Builder interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CseqMap {
    pub(super) fn new() -> Self {
        Self { entries: Vec::new() }
    }
    pub(super) fn reserve(&mut self, additional: usize) {
        self.entries.reserve(additional)
    }
    pub(super) fn insert(&mut self, effect_rid: REffectId, cseq: CseqFull) {
        // Builders should insert every effect max once
        self.entries.push((effect_rid, cseq))
    }
    pub(super) fn retain(&mut self, mut func: impl FnMut(&REffectId, &mut CseqFull) -> bool) {
        self.entries.retain_mut(|(effect_rid, cseq)| func(effect_rid, cseq))
    }
    pub(super) fn clear(&mut self) {
        self.entries.clear()
    }
}
