use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::uad::item::SolSubsystem,
};

pub struct SolSubsystemInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub enabled: bool,
}
impl SolSubsystemInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, enabled: bool) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            enabled,
        }
    }
}
impl From<&SolSubsystem> for SolSubsystemInfo {
    fn from(sol_subsystem: &SolSubsystem) -> Self {
        SolSubsystemInfo::new(
            sol_subsystem.get_id(),
            sol_subsystem.get_type_id(),
            sol_subsystem.get_fit_id(),
            sol_subsystem.get_bool_state(),
        )
    }
}
