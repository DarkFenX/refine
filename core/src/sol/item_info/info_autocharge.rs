use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolAutocharge,
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
impl From<&SolAutocharge> for SolAutoChargeInfo {
    fn from(sol_autocharge: &SolAutocharge) -> Self {
        SolAutoChargeInfo::new(
            sol_autocharge.get_id(),
            sol_autocharge.get_fit_id(),
            sol_autocharge.get_a_item_id(),
            sol_autocharge.cont_id,
        )
    }
}
