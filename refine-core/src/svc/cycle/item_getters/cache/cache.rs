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
    // Actual cseq map of an item + cycling option combination, only for items which cycle. Slots
    // past the used ones are emptied, and kept to avoid allocations
    cseq_slots: Vec<CseqMap>,
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
            cseq_slot.clear();
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
        // If generation matches and data is there already, return it
        let slot_position = item_cell.slot_positions[item_co_code.into_usize()];
        if slot_position.is_slot() {
            return CseqLookup::Built(Some(&self.cseq_slots[slot_position.into_usize()]));
        }
        if slot_position == CseqSlotPosition::NOT_CYCLING {
            return CseqLookup::Built(None);
        }
        // Start calculating cseq - add new empty cseq slot, and reference it
        let slot_position = self.cseq_slots_used;
        item_cell.slot_positions[item_co_code.into_usize()] = CseqSlotPosition::from_usize(slot_position);
        self.cseq_slots_used += 1;
        if slot_position == self.cseq_slots.len() {
            self.cseq_slots.push(CseqMap::new());
        }
        CseqLookup::Started(CseqBuild {
            cache: self,
            item_index,
            item_co_code,
            slot_position,
        })
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

pub(in crate::svc::cycle::item_getters) enum CseqLookup<'c> {
    Built(Option<&'c CseqMap>),
    Started(CseqBuild<'c>),
}

/// Cseq build in progress, the only way to write to its slot.
///
/// Builds can nest (e.g. charges build their containers), so a builder reaches other items'
/// sequences through the cache it holds. The slot and the item cell are kept as positions, since
/// nested builds can reallocate both.
pub(in crate::svc::cycle::item_getters) struct CseqBuild<'c> {
    cache: &'c mut CseqCache,
    item_index: usize,
    item_co_code: ItemCyclingOptionsCode,
    slot_position: usize,
}
impl<'c> CseqBuild<'c> {
    pub(in crate::svc::cycle::item_getters) fn cache(&mut self) -> &mut CseqCache {
        self.cache
    }
    pub(in crate::svc::cycle::item_getters) fn reserve(&mut self, additional: usize) {
        self.cache.cseq_slots[self.slot_position].reserve(additional)
    }
    pub(in crate::svc::cycle::item_getters) fn insert(&mut self, key: REffectId, value: CseqFull) -> Option<CseqFull> {
        self.cache.cseq_slots[self.slot_position].insert(key, value)
    }
    pub(in crate::svc::cycle::item_getters) fn retain(&mut self, func: impl FnMut(&REffectId, &mut CseqFull) -> bool) {
        self.cache.cseq_slots[self.slot_position].retain(func)
    }
    pub(in crate::svc::cycle::item_getters) fn finish(mut self, cycles: bool) -> Option<&'c CseqMap> {
        if !cycles {
            self.finish_not_cycling();
            return None;
        }
        Some(&self.cache.cseq_slots[self.slot_position])
    }
    // Items which don't cycle are marked in their cell, and give the slot back, unless builds
    // nested into this one took slots after it
    // Kept out of line, so that finishing a build of an item which cycles stays small enough to be
    // inlined
    #[cold]
    #[inline(never)]
    fn finish_not_cycling(&mut self) {
        self.cache.item_cells[self.item_index].slot_positions[self.item_co_code.into_usize()] =
            CseqSlotPosition::NOT_CYCLING;
        self.cache.cseq_slots[self.slot_position].clear();
        if self.slot_position + 1 == self.cache.cseq_slots_used {
            self.cache.cseq_slots_used = self.slot_position;
        }
    }
}
