use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::uad::item::{SolService, SolServiceState},
};

pub struct SolServiceInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub state: SolServiceState,
}
impl SolServiceInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, state: SolServiceState) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            state,
        }
    }
}
impl From<&SolService> for SolServiceInfo {
    fn from(sol_service: &SolService) -> Self {
        SolServiceInfo::new(
            sol_service.get_id(),
            sol_service.get_type_id(),
            sol_service.get_fit_id(),
            sol_service.get_service_state(),
        )
    }
}
