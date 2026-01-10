use std::collections::HashMap;

use crate::{
    num::SlotIndex,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::{RMapRSet, RSet},
};

pub struct ValSlotIndexFail {
    /// Map between slot number and multiple items trying to take it.
    pub slot_users: HashMap<SlotIndex, Vec<ItemId>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_implant_slot_index_fast(&self, kfs: &RSet<UItemId>) -> bool {
        validate_slot_index_fast(kfs, &self.slotted_implants)
    }
    pub(in crate::svc::vast) fn validate_booster_slot_index_fast(&self, kfs: &RSet<UItemId>) -> bool {
        validate_slot_index_fast(kfs, &self.slotted_boosters)
    }
    pub(in crate::svc::vast) fn validate_subsystem_slot_index_fast(&self, kfs: &RSet<UItemId>) -> bool {
        validate_slot_index_fast(kfs, &self.slotted_subsystems)
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_implant_slot_index_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValSlotIndexFail> {
        validate_slot_index_verbose(kfs, ctx, &self.slotted_implants)
    }
    pub(in crate::svc::vast) fn validate_booster_slot_index_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValSlotIndexFail> {
        validate_slot_index_verbose(kfs, ctx, &self.slotted_boosters)
    }
    pub(in crate::svc::vast) fn validate_subsystem_slot_index_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValSlotIndexFail> {
        validate_slot_index_verbose(kfs, ctx, &self.slotted_subsystems)
    }
}

fn validate_slot_index_fast(kfs: &RSet<UItemId>, data: &RMapRSet<SlotIndex, UItemId>) -> bool {
    data.values_inner()
        .all(|item_uids| item_uids.len() < 2 || item_uids.is_subset(kfs))
}
fn validate_slot_index_verbose(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    data: &RMapRSet<SlotIndex, UItemId>,
) -> Option<ValSlotIndexFail> {
    let mut slot_users = HashMap::new();
    for (a_slot, users) in data.iter() {
        if users.len() >= 2 {
            let users: Vec<_> = users
                .filter(|item_uid| !kfs.contains(item_uid))
                .map(|item_uid| ctx.u_data.items.xid_by_iid(*item_uid))
                .collect();
            if !users.is_empty() {
                slot_users.insert(*a_slot, users);
            }
        }
    }
    match slot_users.is_empty() {
        true => None,
        false => Some(ValSlotIndexFail { slot_users }),
    }
}
