use super::{
    generation::Generation,
    key::{CACHE_KEY_COUNT, CacheKey},
    short_index::ShortIndex,
};
use crate::{
    rd::REffectId,
    svc::cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq},
    util::{DenseId, RMap},
};

type CseqFull = CycleSeq<CycleDataFull, CSeqHardDtFull>;

/// Cycle sequences of items, built once per item and cycling options.
///
/// Built sequences are kept until the cache is cleared, so the solar system must not change in a
/// way which affects cycle sequences, between clearing the cache and the last read from it.
pub(crate) struct CseqCache {
    // Current generation of cache. Item cells with generation mismatch are considered stale
    generation: Generation,
    // Holds data for every item, reusing its dense ID as index here. The data contains one or more
    // references to cseq slots, which are picked according to cycling options
    item_cells: Vec<ItemCell>,
    // Actual cseq map of an item + cycling option combination. Slots past the used ones are
    // emptied, and kept to avoid allocations
    cseq_slots: Vec<CseqSlot>,
    cseq_slots_used: usize,
    // Two separate cursors: first for reading, second for writing
    cseq_slot_current: usize,
    cseq_slot_building: usize,
}
impl CseqCache {
    pub(crate) fn new() -> Self {
        Self {
            generation: Generation::default(),
            item_cells: Vec::new(),
            cseq_slots: Vec::new(),
            cseq_slots_used: 0,
            cseq_slot_current: 0,
            cseq_slot_building: 0,
        }
    }
    pub(crate) fn clear(&mut self) {
        // Always clear calculated cseqs
        for cseq_slot in &mut self.cseq_slots[..self.cseq_slots_used] {
            cseq_slot.seqs.clear();
        }
        self.cseq_slots_used = 0;
        // Increment generation; if it wrapped, clear cells as well, to make sure false matches do
        // not happen
        if self.generation.increment() {
            self.item_cells.clear();
        }
    }
    // Selection
    pub(in crate::svc::cycle::item_getters) fn select_or_start_build(&mut self, key: CacheKey) -> CseqLookup {
        let (item_uid, code) = key;
        // Reserve enough space to store references for requested item
        let item_index = item_uid.index();
        if item_index >= self.item_cells.len() {
            self.item_cells.resize(item_index + 1, ItemCell::DEFAULT_EMPTY);
        }
        let item_cell = &mut self.item_cells[item_index];
        // On generation mismatch, invalidate references for the item
        if item_cell.generation != self.generation {
            *item_cell = ItemCell::make_empty(self.generation);
        }
        let slot_position = item_cell.slot_positions[code as usize];
        // If generation matches and data is there already, return it
        if slot_position != ShortIndex::NO_SLOT {
            self.cseq_slot_current = slot_position.into_usize();
            return CseqLookup::Built(self.cseq_slots[self.cseq_slot_current].cycles);
        }
        // Start calculating cseq - add new empty cseq slot, and reference it
        let slot_position = self.cseq_slots_used;
        item_cell.slot_positions[code as usize] = ShortIndex::from_usize(slot_position);
        self.cseq_slots_used += 1;
        if slot_position == self.cseq_slots.len() {
            self.cseq_slots.push(CseqSlot::new());
        }
        // Builds can nest (e.g. charges build their containers), so the build in progress is
        // resumed once this one finishes
        let parent = std::mem::replace(&mut self.cseq_slot_building, slot_position);
        CseqLookup::Started(CseqBuild { slot_position, parent })
    }
    pub(in crate::svc::cycle::item_getters) fn finish_build(&mut self, build: CseqBuild, cycles: bool) {
        let slot = &mut self.cseq_slots[build.slot_position];
        slot.cycles = cycles;
        if !cycles {
            slot.seqs.clear();
        }
        self.cseq_slot_building = build.parent;
        self.cseq_slot_current = build.slot_position;
    }
    // Building
    pub(in crate::svc) fn reserve(&mut self, additional: usize) {
        self.cseq_slots[self.cseq_slot_building].seqs.reserve(additional)
    }
    pub(in crate::svc) fn insert(&mut self, key: REffectId, value: CseqFull) -> Option<CseqFull> {
        self.cseq_slots[self.cseq_slot_building].seqs.insert(key, value)
    }
    pub(in crate::svc) fn retain(&mut self, func: impl FnMut(&REffectId, &mut CseqFull) -> bool) {
        self.cseq_slots[self.cseq_slot_building].seqs.retain(func)
    }
    // Reading
    pub(in crate::svc) fn get(&self, key: &REffectId) -> Option<&CseqFull> {
        self.cseq_slots[self.cseq_slot_current].seqs.get(key)
    }
    pub(in crate::svc) fn iter(&self) -> impl ExactSizeIterator<Item = (&REffectId, &CseqFull)> {
        self.cseq_slots[self.cseq_slot_current].seqs.iter()
    }
}

#[derive(Copy, Clone)]
struct ItemCell {
    generation: Generation,
    slot_positions: [ShortIndex; CACHE_KEY_COUNT],
}
impl ItemCell {
    const DEFAULT_EMPTY: Self = Self::make_empty(Generation::default());
    const fn make_empty(generation: Generation) -> Self {
        Self {
            generation,
            slot_positions: [ShortIndex::NO_SLOT; CACHE_KEY_COUNT],
        }
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
    slot_position: usize,
    parent: usize,
}
