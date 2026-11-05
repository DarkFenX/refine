use std::collections::hash_map::Entry;

use crate::{
    rd::REffectId,
    svc::cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq},
    ud::UItemId,
    util::RMap,
};

type CseqFull = CycleSeq<CycleDataFull, CSeqHardDtFull>;
type CacheKey = (UItemId, u8);

/// Cycle sequences of items, built once per item and cycling options.
///
/// Built sequences are kept until the cache is cleared, so the solar system must not change between
/// clearing it and the last read from it. Reads go to the item requested last.
pub(crate) struct CseqCache {
    slot_positions: RMap<CacheKey, usize>,
    // Slots past the used ones are emptied, and kept to avoid allocations
    slots: Vec<CseqSlot>,
    used: usize,
    current: usize,
    building: usize,
}
impl CseqCache {
    pub(crate) fn new() -> Self {
        Self {
            slot_positions: RMap::new(),
            slots: Vec::new(),
            used: 0,
            current: 0,
            building: 0,
        }
    }
    pub(crate) fn clear(&mut self) {
        for slot in &mut self.slots[..self.used] {
            slot.seqs.clear();
        }
        self.used = 0;
        self.slot_positions.clear();
    }
    // Selection
    pub(in crate::svc::cycle::item_getters) fn select_or_start_build(&mut self, key: CacheKey) -> CseqLookup {
        match self.slot_positions.entry(key) {
            Entry::Occupied(entry) => {
                self.current = *entry.get();
                CseqLookup::Built(self.slots[self.current].cycles)
            }
            Entry::Vacant(entry) => {
                let slot = self.used;
                entry.insert(slot);
                self.used += 1;
                if slot == self.slots.len() {
                    self.slots.push(CseqSlot::new());
                }
                // Builds nest (charges build their containers), so the build in progress is
                // resumed once this one finishes
                let parent = std::mem::replace(&mut self.building, slot);
                CseqLookup::Started(CseqBuild { slot, parent })
            }
        }
    }
    pub(in crate::svc::cycle::item_getters) fn finish_build(&mut self, build: CseqBuild, cycles: bool) {
        let slot = &mut self.slots[build.slot];
        slot.cycles = cycles;
        if !cycles {
            slot.seqs.clear();
        }
        self.building = build.parent;
        self.current = build.slot;
    }
    // Building
    pub(in crate::svc) fn reserve(&mut self, additional: usize) {
        self.slots[self.building].seqs.reserve(additional)
    }
    pub(in crate::svc) fn insert(&mut self, key: REffectId, value: CseqFull) -> Option<CseqFull> {
        self.slots[self.building].seqs.insert(key, value)
    }
    pub(in crate::svc) fn retain(&mut self, func: impl FnMut(&REffectId, &mut CseqFull) -> bool) {
        self.slots[self.building].seqs.retain(func)
    }
    // Reading
    pub(in crate::svc) fn get(&self, key: &REffectId) -> Option<&CseqFull> {
        self.slots[self.current].seqs.get(key)
    }
    pub(in crate::svc) fn iter(&self) -> impl ExactSizeIterator<Item = (&REffectId, &CseqFull)> {
        self.slots[self.current].seqs.iter()
    }
}

struct CseqSlot {
    cycles: bool,
    seqs: RMap<REffectId, CseqFull>,
}
impl CseqSlot {
    fn new() -> Self {
        Self {
            cycles: false,
            seqs: RMap::new(),
        }
    }
}

pub(in crate::svc::cycle::item_getters) enum CseqLookup {
    Built(bool),
    Started(CseqBuild),
}

pub(in crate::svc::cycle::item_getters) struct CseqBuild {
    slot: usize,
    parent: usize,
}
