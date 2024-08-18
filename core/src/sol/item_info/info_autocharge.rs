use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolAutocharge,
};

pub struct SolAutoChargeInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub type_id: EItemId,
    pub cont_id: SolItemId,
    pub enabled: bool,
}
impl SolAutoChargeInfo {
    fn new(id: SolItemId, fit_id: SolFitId, type_id: EItemId, cont_id: SolItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            cont_id,
            enabled,
        }
    }
}
impl From<&SolAutocharge> for SolAutoChargeInfo {
    fn from(sol_autocharge: &SolAutocharge) -> Self {
        // No projections because they fully match to projections of parent item
        SolAutoChargeInfo::new(
            sol_autocharge.get_id(),
            sol_autocharge.get_fit_id(),
            sol_autocharge.get_type_id(),
            sol_autocharge.get_cont_id(),
            !sol_autocharge.get_force_disable(),
        )
    }
}
