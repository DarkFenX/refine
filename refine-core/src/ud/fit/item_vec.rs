use std::collections::BTreeMap;

use itertools::Itertools;

use crate::{num::Index, ud::UItemId};

#[derive(Clone)]
pub(crate) struct UItemVec {
    pub(super) data: BTreeMap<Index, UItemId>,
}
impl UItemVec {
    pub(in crate::ud::fit) fn new() -> Self {
        Self { data: BTreeMap::new() }
    }
    pub(crate) fn iter_all(&self) -> impl ExactSizeIterator<Item = Option<UItemId>> {
        SlotIter::new(
            self.data.iter().map(|(pos, item_uid)| (*pos, *item_uid)),
            self.slot_count(),
        )
    }
    pub(crate) fn iter_uids(&self) -> impl ExactSizeIterator<Item = UItemId> {
        self.data.values().copied()
    }
    pub(crate) fn iter_uids_from_pos(&self, start_pos: Index) -> impl Iterator<Item = UItemId> {
        self.data.range(start_pos..).map(|(_, item_uid)| *item_uid)
    }
    pub(crate) fn get(&self, pos: Index) -> Option<UItemId> {
        self.data.get(&pos).copied()
    }
    pub(crate) fn slot_count(&self) -> usize {
        match self.data.last_key_value() {
            Some((pos, _)) => pos.into_usize() + 1,
            None => 0,
        }
    }
    pub(crate) fn item_count(&self) -> usize {
        self.data.len()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Addition
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemVec {
    pub(crate) fn append(&mut self, item_uid: UItemId) -> Index {
        let pos = Index::from_usize(self.slot_count());
        self.data.insert(pos, item_uid);
        pos
    }
    pub(crate) fn equip(&mut self, item_uid: UItemId) -> Index {
        let mut pos = Index::ZERO;
        for &iter_pos in self.data.keys() {
            if iter_pos > pos {
                break;
            }
            pos = iter_pos + Index::ONE;
        }
        self.data.insert(pos, item_uid);
        pos
    }
    // Returns item UIDs for items which need their positions shifted right
    pub(crate) fn insert(&mut self, pos: Index, item_uid: UItemId) -> impl ExactSizeIterator<Item = UItemId> {
        // Update positions of all elements on requested position and past it, to make space for the
        // new one
        let shifts = self
            .data
            .range(pos..)
            .map(|(iter_pos, iter_item_uid)| (*iter_pos, *iter_item_uid))
            .collect_vec();
        for shift in shifts.iter().rev() {
            self.data.remove(&shift.0).unwrap();
            self.data.insert(shift.0 + Index::ONE, shift.1);
        }
        // Insert element itself
        self.data.insert(pos, item_uid);
        shifts.into_iter().map(|(_, item_uid)| item_uid)
    }
    pub(crate) fn place(&mut self, pos: Index, item_uid: UItemId) {
        self.data.insert(pos, item_uid);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Changing
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemVec {
    // Returns item UIDs and shift direction
    pub(crate) fn shift(
        &mut self,
        src_pos: Index,
        tgt_pos: Index,
    ) -> Option<(impl ExactSizeIterator<Item = UItemId>, UItemVecShiftDir)> {
        let item_uid = self.data.remove(&src_pos);
        let (shifts, dir) = match tgt_pos.cmp(&src_pos) {
            std::cmp::Ordering::Greater => {
                let shifts = self
                    .data
                    .range(src_pos + Index::ONE..=tgt_pos)
                    .map(|(iter_pos, iter_item_uid)| (*iter_pos, *iter_item_uid))
                    .collect_vec();
                for shift in shifts.iter() {
                    self.data.remove(&shift.0).unwrap();
                    self.data.insert(shift.0 - Index::ONE, shift.1);
                }
                (shifts, UItemVecShiftDir::Left)
            }
            std::cmp::Ordering::Equal => return None,
            std::cmp::Ordering::Less => {
                let shifts = self
                    .data
                    .range(tgt_pos..src_pos)
                    .map(|(iter_pos, iter_item_uid)| (*iter_pos, *iter_item_uid))
                    .collect_vec();
                for shift in shifts.iter().rev() {
                    self.data.remove(&shift.0).unwrap();
                    self.data.insert(shift.0 + Index::ONE, shift.1);
                }
                (shifts, UItemVecShiftDir::Right)
            }
        };
        if let Some(item_uid) = item_uid {
            self.data.insert(tgt_pos, item_uid);
        }
        Some((shifts.into_iter().map(|(_, item_uid)| item_uid), dir))
    }
    // Returns item ID of target item, if there is one
    pub(crate) fn swap(&mut self, src_pos: Index, tgt_pos: Index) -> Option<UItemId> {
        let src_item_uid = self.data.remove(&src_pos);
        let tgt_item_uid = self.data.remove(&tgt_pos);
        if let Some(src_item_uid) = src_item_uid {
            self.data.insert(tgt_pos, src_item_uid);
        }
        if let Some(tgt_item_uid) = tgt_item_uid {
            self.data.insert(src_pos, tgt_item_uid);
        }
        tgt_item_uid
    }
}

pub(crate) enum UItemVecShiftDir {
    Left,
    Right,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Removal
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemVec {
    pub(crate) fn free(&mut self, pos: Index) {
        self.data.remove(&pos);
    }
    // Returns item UIDs for items which need their positions shifted left
    pub(crate) fn remove(&mut self, pos: Index) -> impl ExactSizeIterator<Item = UItemId> {
        self.data.remove(&pos);
        // Update positions of all elements on positions to the right of requested one
        let shifts = self
            .data
            .range(pos + Index::ONE..)
            .map(|(iter_pos, iter_item_uid)| (*iter_pos, *iter_item_uid))
            .collect_vec();
        for shift in shifts.iter() {
            self.data.remove(&shift.0).unwrap();
            self.data.insert(shift.0 - Index::ONE, shift.1);
        }
        shifts.into_iter().map(|(_, item_uid)| item_uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Slot iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
struct SlotIter<I>
where
    I: Iterator<Item = (Index, UItemId)>,
{
    taken_slot_iter: I,
    next_taken: Option<(Index, UItemId)>,
    current: Index,
    slot_count: usize,
}
impl<I> SlotIter<I>
where
    I: Iterator<Item = (Index, UItemId)>,
{
    fn new(mut taken_slot_iter: I, slot_count: usize) -> Self {
        let next_taken = taken_slot_iter.next();
        Self {
            taken_slot_iter,
            next_taken,
            current: Index::ZERO,
            slot_count,
        }
    }
}
impl<I> Iterator for SlotIter<I>
where
    I: Iterator<Item = (Index, UItemId)>,
{
    type Item = Option<UItemId>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_taken {
            Some((taken_pos, item_uid)) => {
                let mut value = None;
                if self.current == taken_pos {
                    value = Some(item_uid);
                    self.next_taken = self.taken_slot_iter.next();
                }
                self.current += Index::ONE;
                Some(value)
            }
            None => None,
        }
    }
}
impl<I> ExactSizeIterator for SlotIter<I>
where
    I: Iterator<Item = (Index, UItemId)>,
{
    fn len(&self) -> usize {
        self.slot_count
    }
}
