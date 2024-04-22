use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolSubsystem,
};

pub struct SolSubsystemInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SolSubsystemInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SolSubsystem> for SolSubsystemInfo {
    fn from(sol_subsystem: &SolSubsystem) -> Self {
        SolSubsystemInfo::new(
            sol_subsystem.id,
            sol_subsystem.fit_id,
            sol_subsystem.a_item_id,
            sol_subsystem.get_bool_state(),
        )
    }
}
