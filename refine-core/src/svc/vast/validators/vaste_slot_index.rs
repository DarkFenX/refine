use itertools::Itertools;

use crate::{
    num::SlotIndex,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::{RMapRSet, RSet},
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
pub struct ValSlotIndexFail {
    /// Slot number and items trying to take it.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::Map<_, _>"))]
    pub slot_users: Vec<(SlotIndex, Vec<ItemId>)>,
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
    let mut slot_users = Vec::new();
    for (slot_index, users) in data.iter() {
        if users.len() >= 2 {
            let users = users
                .filter_map(|item_uid| match kfs.contains(item_uid) {
                    true => None,
                    false => Some(ctx.u_data.items.ext_id_by_int_id(*item_uid)),
                })
                .collect_vec();
            if !users.is_empty() {
                slot_users.push((*slot_index, users));
            }
        }
    }
    match slot_users.is_empty() {
        true => None,
        false => Some(ValSlotIndexFail { slot_users }),
    }
}
