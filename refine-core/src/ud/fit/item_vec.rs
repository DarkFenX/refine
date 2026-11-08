use std::{collections::BTreeMap, ops::Bound};

use itertools::Itertools;

use crate::{RackPos, ud::UItemId};

#[derive(Clone)]
pub(crate) struct UItemVec {
    data: BTreeMap<RackPos, UItemId>,
}
impl UItemVec {
    pub(in crate::ud::fit) fn new() -> Self {
        Self { data: BTreeMap::new() }
    }
    pub(crate) fn iter_uids(&self) -> impl ExactSizeIterator<Item = UItemId> {
        self.data.values().copied()
    }
    pub(crate) fn iter_uids_from_pos(&self, start_pos: RackPos) -> impl Iterator<Item = UItemId> {
        self.data.range(start_pos..).map(|(_, item_uid)| *item_uid)
    }
    pub(crate) fn get(&self, pos: RackPos) -> Option<UItemId> {
        self.data.get(&pos).copied()
    }
    pub(crate) fn slot_count(&self) -> usize {
        match self.data.last_key_value() {
            Some((pos, _)) => pos.into_usize() + 1,
            None => 0,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Addition
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemVec {
    pub(crate) fn get_append_pos(&self) -> Option<RackPos> {
        RackPos::from_usize_checked(self.slot_count()).ok()
    }
    pub(crate) fn get_equip_pos(&self) -> Option<RackPos> {
        let mut pos = 0;
        for &iter_pos in self.data.keys() {
            let iter_pos = iter_pos.into_usize();
            if iter_pos > pos {
                break;
            }
            pos = iter_pos + 1;
        }
        RackPos::from_usize_checked(pos).ok()
    }
    /// Insertion shifts items on and after its position right, which fails only when the last
    /// position is taken
    pub(crate) fn can_insert(&self) -> bool {
        !self.data.contains_key(&RackPos::MAX)
    }
    /// Returns item UIDs for items which need their positions shifted right
    pub(crate) fn insert(&mut self, pos: RackPos, item_uid: UItemId) -> impl ExactSizeIterator<Item = UItemId> {
        // Update positions of all elements on requested position and past it, to make space for the
        // new one
        let shifts = self
            .data
            .range(pos..)
            .map(|(iter_pos, iter_item_uid)| (*iter_pos, *iter_item_uid))
            .collect_vec();
        for shift in shifts.iter().rev() {
            self.data.remove(&shift.0).unwrap();
            self.data.insert(shift.0.inc_unchecked(), shift.1);
        }
        // Insert element itself
        self.data.insert(pos, item_uid);
        shifts.into_iter().map(|(_, item_uid)| item_uid)
    }
    pub(crate) fn place(&mut self, pos: RackPos, item_uid: UItemId) {
        self.data.insert(pos, item_uid);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Changing
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemVec {
    /// Returns item UIDs and shift direction
    pub(crate) fn shift(
        &mut self,
        src_pos: RackPos,
        tgt_pos: RackPos,
    ) -> Option<(impl ExactSizeIterator<Item = UItemId>, UItemVecShiftDir)> {
        let item_uid = self.data.remove(&src_pos);
        let (shifts, dir) = match tgt_pos.cmp(&src_pos) {
            std::cmp::Ordering::Greater => {
                let shifts = self
                    .data
                    .range((Bound::Excluded(src_pos), Bound::Included(tgt_pos)))
                    .map(|(iter_pos, iter_item_uid)| (*iter_pos, *iter_item_uid))
                    .collect_vec();
                for shift in shifts.iter() {
                    self.data.remove(&shift.0).unwrap();
                    self.data.insert(shift.0.dec_unchecked(), shift.1);
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
                    self.data.insert(shift.0.inc_unchecked(), shift.1);
                }
                (shifts, UItemVecShiftDir::Right)
            }
        };
        if let Some(item_uid) = item_uid {
            self.data.insert(tgt_pos, item_uid);
        }
        Some((shifts.into_iter().map(|(_, item_uid)| item_uid), dir))
    }
    /// Returns item ID of target item, if there is one
    pub(crate) fn swap(&mut self, src_pos: RackPos, tgt_pos: RackPos) -> Option<UItemId> {
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
    pub(crate) fn free(&mut self, pos: RackPos) {
        self.data.remove(&pos);
    }
    /// Returns item UIDs for items which need their positions shifted left
    pub(crate) fn remove(&mut self, pos: RackPos) -> impl ExactSizeIterator<Item = UItemId> {
        self.data.remove(&pos);
        // Update positions of all elements on positions to the right of requested one
        let shifts = self
            .data
            .range((Bound::Excluded(pos), Bound::Unbounded))
            .map(|(iter_pos, iter_item_uid)| (*iter_pos, *iter_item_uid))
            .collect_vec();
        for shift in shifts.iter() {
            self.data.remove(&shift.0).unwrap();
            self.data.insert(shift.0.dec_unchecked(), shift.1);
        }
        shifts.into_iter().map(|(_, item_uid)| item_uid)
    }
}
