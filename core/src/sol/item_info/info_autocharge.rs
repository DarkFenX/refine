use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolAutoCharge,
};

pub struct SolAutoChargeInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub cont_id: SolItemId,
}
impl SolAutoChargeInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, cont_id: SolItemId) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            cont_id,
        }
    }
}
impl From<&SolAutoCharge> for SolAutoChargeInfo {
    fn from(sol_autocharge: &SolAutoCharge) -> Self {
        SolAutoChargeInfo::new(
            sol_autocharge.base.id,
            sol_autocharge.fit_id,
            sol_autocharge.base.a_item_id,
            sol_autocharge.cont_id,
        )
    }
}
