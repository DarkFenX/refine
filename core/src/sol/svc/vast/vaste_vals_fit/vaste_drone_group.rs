use itertools::Itertools;

use crate::{
    defs::{EItemGrpId, SolItemId},
    sol::svc::vast::SolVastFitData,
    util::StSet,
};

pub struct SolValDroneGroupFail {
    pub allowed_group_ids: Vec<EItemGrpId>,
    pub items: Vec<SolValDroneGroupItemInfo>,
}

#[derive(Copy, Clone)]
pub struct SolValDroneGroupItemInfo {
    pub item_id: SolItemId,
    pub group_id: EItemGrpId,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_drone_group_fast(&mut self, kfs: &StSet<SolItemId>) -> bool {
        match kfs.is_empty() {
            true => self.drone_group_mismatches.is_empty(),
            false => self.drone_group_mismatches.difference(kfs).nth(0).is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_drone_group_verbose(
        &mut self,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValDroneGroupFail> {
        if self.drone_group_mismatches.is_empty() {
            return None;
        }
        let items = self
            .drone_group_mismatches
            .values()
            .filter(|v| !kfs.contains(&v.item_id))
            .copied()
            .collect_vec();
        if items.is_empty() {
            return None;
        }
        Some(SolValDroneGroupFail {
            allowed_group_ids: self.drone_group_limit.clone(),
            items,
        })
    }
}
