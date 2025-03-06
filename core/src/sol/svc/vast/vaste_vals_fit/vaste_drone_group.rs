use crate::{
    defs::{EItemGrpId, SolItemId},
    sol::svc::vast::SolVastFitData,
};

pub struct SolValDroneGroupFail {
    pub allowed_group_ids: Vec<EItemGrpId>,
    pub items: Vec<SolValDroneGroupItemInfo>,
}
impl SolValDroneGroupFail {
    fn new(allowed_group_ids: Vec<EItemGrpId>, items: Vec<SolValDroneGroupItemInfo>) -> Self {
        Self {
            allowed_group_ids,
            items,
        }
    }
}

#[derive(Copy, Clone)]
pub struct SolValDroneGroupItemInfo {
    pub item_id: SolItemId,
    pub group_id: EItemGrpId,
}
impl SolValDroneGroupItemInfo {
    pub(in crate::sol::svc::vast) fn new(item_id: SolItemId, group_id: EItemGrpId) -> Self {
        Self { item_id, group_id }
    }
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
        Some(SolValDroneGroupFail::new(
            self.drone_group_limit.clone(),
            self.drone_group_mismatches.values().copied().collect(),
        ))
    }
}
