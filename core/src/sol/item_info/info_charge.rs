use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolCharge,
};

pub struct SolChargeInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub cont_id: SolItemId,
    pub enabled: bool,
    // No projections because they fully match to projections of parent item
}
impl SolChargeInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, cont_id: SolItemId, enabled: bool) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            cont_id,
            enabled,
        }
    }
}
impl From<&SolCharge> for SolChargeInfo {
    fn from(sol_charge: &SolCharge) -> Self {
        SolChargeInfo::new(
            sol_charge.get_id(),
            sol_charge.get_type_id(),
            sol_charge.get_fit_id(),
            sol_charge.get_cont_id(),
            !sol_charge.get_force_disable(),
        )
    }
}
