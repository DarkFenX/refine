use super::{
    cseq_slot_position::CseqSlotPosition,
    generation::Generation,
    item_co_code::ItemCyclingOptionsCode,
    key::{CacheKey, ItemCyclingOptions},
};
use crate::{
    rd::REffectId,
    svc::cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq},
    ud::UItemId,
    util::{DenseId, RMap},
};

type CseqFull = CycleSeq<CycleDataFull, CSeqHardDtFull>;
/// Cycle sequences of an item's effects.
pub(in crate::svc) type CseqMap = RMap<REffectId, CseqFull>;

/// Cycle sequences of items, built once per item and cycling options.
///
/// Built sequences are kept until the cache is cleared, so the solar system must not change in a
/// way which affects cycle sequences, between clearing the cache and the last read from it.
pub(crate) struct CseqCache {
    // Current generation of cache. Item cells with generation mismatch are considered stale
    generation: Generation,
    // Holds data for every item, reusing its dense ID as index here. The data contains one or more
    // references to cseq slots, which are picked by item cycling options code
    item_cells: Vec<ItemCell>,
    // Actual cseq map of an item + cycling option combination. Slots past the used ones are
    // emptied, and kept to avoid allocations
    cseq_slots: Vec<CseqSlot>,
    cseq_slots_used: usize,
}
impl CseqCache {
    pub(crate) fn new() -> Self {
        Self {
            generation: Generation::default(),
            item_cells: Vec::new(),
            cseq_slots: Vec::new(),
            cseq_slots_used: 0,
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
    pub(in crate::svc::cycle::item_getters) fn get_or_build<C, F>(
        &mut self,
        item_uid: UItemId,
        item_co: C,
        builder: F,
    ) -> Option<&CseqMap>
    where
        C: ItemCyclingOptions,
        F: FnOnce(&mut CseqBuild, C) -> bool,
    {
        let mut cseq_build = match self.select_or_start_build(item_co.cache_key(item_uid)) {
            CseqLookup::Built(cseq_map) => return cseq_map,
            CseqLookup::Started(cseq_build) => cseq_build,
        };
        // The builder gets the cycling options the key was made of, and nothing else
        let cycles = builder(&mut cseq_build, item_co);
        cseq_build.finish(cycles)
    }
    #[inline(always)]
    pub(in crate::svc::cycle::item_getters) fn select_or_start_build(&mut self, key: CacheKey) -> CseqLookup<'_> {
        let (item_uid, item_co_code) = key;
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
        let slot_position = item_cell.slot_positions[item_co_code.into_usize()];
        // If generation matches and data is there already, return it
        if slot_position != CseqSlotPosition::NONE {
            return CseqLookup::Built(self.get_cseq_map(slot_position.into_usize()));
        }
        // Start calculating cseq - add new empty cseq slot, and reference it
        let slot_position = self.cseq_slots_used;
        item_cell.slot_positions[item_co_code.into_usize()] = CseqSlotPosition::from_usize(slot_position);
        self.cseq_slots_used += 1;
        if slot_position == self.cseq_slots.len() {
            self.cseq_slots.push(CseqSlot::new());
        }
        CseqLookup::Started(CseqBuild {
            cache: self,
            slot_position,
        })
    }
    fn get_cseq_map(&self, slot_position: usize) -> Option<&CseqMap> {
        let cseq_slot = &self.cseq_slots[slot_position];
        match cseq_slot.cycles {
            true => Some(&cseq_slot.seqs),
            false => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helper data entities
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
struct ItemCell {
    generation: Generation,
    slot_positions: [CseqSlotPosition; ItemCyclingOptionsCode::COUNT],
}
impl ItemCell {
    const DEFAULT_EMPTY: Self = Self::make_empty(Generation::default());
    const fn make_empty(generation: Generation) -> Self {
        Self {
            generation,
            slot_positions: [CseqSlotPosition::NONE; ItemCyclingOptionsCode::COUNT],
        }
    }
}

struct CseqSlot {
    cycles: bool,
    seqs: CseqMap,
}
impl CseqSlot {
    fn new() -> Self {
        Self {
            cycles: false,
            seqs: RMap::new(),
        }
    }
}

pub(in crate::svc::cycle::item_getters) enum CseqLookup<'c> {
    Built(Option<&'c CseqMap>),
    Started(CseqBuild<'c>),
}

/// Cseq build in progress, the only way to write to its slot.
///
/// Builds can nest (e.g. charges build their containers), so a builder reaches other items'
/// sequences through the cache it holds. The slot is kept as a position, since nested builds can
/// reallocate slots.
pub(in crate::svc::cycle::item_getters) struct CseqBuild<'c> {
    cache: &'c mut CseqCache,
    slot_position: usize,
}
impl<'c> CseqBuild<'c> {
    pub(in crate::svc::cycle::item_getters) fn cache(&mut self) -> &mut CseqCache {
        self.cache
    }
    pub(in crate::svc::cycle::item_getters) fn reserve(&mut self, additional: usize) {
        self.cache.cseq_slots[self.slot_position].seqs.reserve(additional)
    }
    pub(in crate::svc::cycle::item_getters) fn insert(&mut self, key: REffectId, value: CseqFull) -> Option<CseqFull> {
        self.cache.cseq_slots[self.slot_position].seqs.insert(key, value)
    }
    pub(in crate::svc::cycle::item_getters) fn retain(&mut self, func: impl FnMut(&REffectId, &mut CseqFull) -> bool) {
        self.cache.cseq_slots[self.slot_position].seqs.retain(func)
    }
    pub(in crate::svc::cycle::item_getters) fn finish(self, cycles: bool) -> Option<&'c CseqMap> {
        let cseq_slot = &mut self.cache.cseq_slots[self.slot_position];
        cseq_slot.cycles = cycles;
        if !cycles {
            cseq_slot.seqs.clear();
            return None;
        }
        Some(&cseq_slot.seqs)
    }
}
