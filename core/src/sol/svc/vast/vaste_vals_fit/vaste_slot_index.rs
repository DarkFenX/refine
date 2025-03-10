use itertools::Itertools;

use crate::{
    defs::{SlotIndex, SolItemId},
    sol::svc::vast::SolVastFitData,
    util::{StMapSetL1, StSet},
};

pub struct SolValSlotIndexFail {
    pub slot: SlotIndex,
    pub users: Vec<SolItemId>,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_implant_slot_index_fast(&self, kfs: &StSet<SolItemId>) -> bool {
        validate_slot_index_fast(&kfs, &self.slotted_implants)
    }
    pub(in crate::sol::svc::vast) fn validate_booster_slot_index_fast(&self, kfs: &StSet<SolItemId>) -> bool {
        validate_slot_index_fast(&kfs, &self.slotted_boosters)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_index_fast(&self, kfs: &StSet<SolItemId>) -> bool {
        validate_slot_index_fast(&kfs, &self.slotted_subsystems)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_implant_slot_index_verbose(
        &self,
        kfs: &StSet<SolItemId>,
    ) -> Vec<SolValSlotIndexFail> {
        validate_slot_index_verbose(kfs, &self.slotted_implants)
    }
    pub(in crate::sol::svc::vast) fn validate_booster_slot_index_verbose(
        &self,
        kfs: &StSet<SolItemId>,
    ) -> Vec<SolValSlotIndexFail> {
        validate_slot_index_verbose(kfs, &self.slotted_boosters)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_index_verbose(
        &self,
        kfs: &StSet<SolItemId>,
    ) -> Vec<SolValSlotIndexFail> {
        validate_slot_index_verbose(kfs, &self.slotted_subsystems)
    }
}

fn validate_slot_index_fast(kfs: &StSet<SolItemId>, data: &StMapSetL1<SlotIndex, SolItemId>) -> bool {
    data.values_inner()
        .all(|item_ids| item_ids.len() < 2 || item_ids.is_subset(kfs))
}
fn validate_slot_index_verbose(
    kfs: &StSet<SolItemId>,
    data: &StMapSetL1<SlotIndex, SolItemId>,
) -> Vec<SolValSlotIndexFail> {
    let mut fails = Vec::new();
    for (slot, users) in data.iter() {
        if users.len() >= 2 {
            let users = users.filter(|v| !kfs.contains(v)).copied().collect_vec();
            if !users.is_empty() {
                fails.push(SolValSlotIndexFail { slot: *slot, users })
            }
        }
    }
    fails
}
