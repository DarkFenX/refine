use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolAutocharge,
};

pub struct SolAutoChargeInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub type_id: EItemId,
    pub cont_id: SolItemId,
}
impl SolAutoChargeInfo {
    fn new(id: SolItemId, fit_id: SolFitId, type_id: EItemId, cont_id: SolItemId) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            cont_id,
        }
    }
}
impl From<&SolAutocharge> for SolAutoChargeInfo {
    fn from(sol_autocharge: &SolAutocharge) -> Self {
        SolAutoChargeInfo::new(
            sol_autocharge.get_id(),
            sol_autocharge.get_fit_id(),
            sol_autocharge.get_type_id(),
            sol_autocharge.get_cont_id(),
        )
    }
}
