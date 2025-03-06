use crate::{
    defs::{EItemGrpId, SolItemId},
    sol::svc::vast::SolVastFitData,
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
    pub(in crate::sol::svc::vast) fn validate_drone_group_fast(&mut self) -> bool {
        self.drone_group_mismatches.is_empty()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_drone_group_verbose(&mut self) -> Option<SolValDroneGroupFail> {
        if self.drone_group_mismatches.is_empty() {
            return None;
        }
        Some(SolValDroneGroupFail {
            allowed_group_ids: self.drone_group_limit.clone(),
            items: self.drone_group_mismatches.values().copied().collect(),
        })
    }
}
