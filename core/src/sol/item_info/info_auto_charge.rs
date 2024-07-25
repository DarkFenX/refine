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
    fn from(sol_auto_charge: &SolAutoCharge) -> Self {
        SolAutoChargeInfo::new(
            sol_auto_charge.base.id,
            sol_auto_charge.fit_id,
            sol_auto_charge.base.a_item_id,
            sol_auto_charge.cont_id,
        )
    }
}
