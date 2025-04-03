use itertools::Itertools;

use crate::{
    ad,
    sol::{ItemId, SlotIndex, svc::vast::VastFitData},
    util::{HMapHSet, HSet},
};

pub struct ValSlotIndexFail {
    pub slot: SlotIndex,
    pub users: Vec<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_implant_slot_index_fast(&self, kfs: &HSet<ItemId>) -> bool {
        validate_slot_index_fast(kfs, &self.slotted_implants)
    }
    pub(in crate::sol::svc::vast) fn validate_booster_slot_index_fast(&self, kfs: &HSet<ItemId>) -> bool {
        validate_slot_index_fast(kfs, &self.slotted_boosters)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_index_fast(&self, kfs: &HSet<ItemId>) -> bool {
        validate_slot_index_fast(kfs, &self.slotted_subsystems)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_implant_slot_index_verbose(
        &self,
        kfs: &HSet<ItemId>,
    ) -> Vec<ValSlotIndexFail> {
        validate_slot_index_verbose(kfs, &self.slotted_implants)
    }
    pub(in crate::sol::svc::vast) fn validate_booster_slot_index_verbose(
        &self,
        kfs: &HSet<ItemId>,
    ) -> Vec<ValSlotIndexFail> {
        validate_slot_index_verbose(kfs, &self.slotted_boosters)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_index_verbose(
        &self,
        kfs: &HSet<ItemId>,
    ) -> Vec<ValSlotIndexFail> {
        validate_slot_index_verbose(kfs, &self.slotted_subsystems)
    }
}

fn validate_slot_index_fast(kfs: &HSet<ItemId>, data: &HMapHSet<SlotIndex, ItemId>) -> bool {
    data.values_inner()
        .all(|item_ids| item_ids.len() < 2 || item_ids.is_subset(kfs))
}
fn validate_slot_index_verbose(kfs: &HSet<ItemId>, data: &HMapHSet<ad::ASlotIndex, ItemId>) -> Vec<ValSlotIndexFail> {
    let mut fails = Vec::new();
    for (a_slot, users) in data.iter() {
        if users.len() >= 2 {
            let users = users.filter(|v| !kfs.contains(v)).copied().collect_vec();
            if !users.is_empty() {
                fails.push(ValSlotIndexFail { slot: *a_slot, users })
            }
        }
    }
    fails
}
