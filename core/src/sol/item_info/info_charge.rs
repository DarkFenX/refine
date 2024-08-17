use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolCharge,
};

pub struct SolChargeInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub type_id: EItemId,
    pub cont_id: SolItemId,
}
impl SolChargeInfo {
    fn new(id: SolItemId, fit_id: SolFitId, type_id: EItemId, cont_id: SolItemId) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            cont_id,
        }
    }
}
impl From<&SolCharge> for SolChargeInfo {
    fn from(sol_charge: &SolCharge) -> Self {
        SolChargeInfo::new(
            sol_charge.get_id(),
            sol_charge.get_fit_id(),
            sol_charge.get_type_id(),
            sol_charge.get_cont_id(),
        )
    }
}
