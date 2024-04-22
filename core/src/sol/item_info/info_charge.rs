use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolCharge,
};

pub struct SolChargeInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub cont_id: SolItemId,
}
impl SolChargeInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, cont_id: SolItemId) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            cont_id,
        }
    }
}
impl From<&SolCharge> for SolChargeInfo {
    fn from(sol_charge: &SolCharge) -> Self {
        SolChargeInfo::new(
            sol_charge.id,
            sol_charge.fit_id,
            sol_charge.a_item_id,
            sol_charge.cont_id,
        )
    }
}
