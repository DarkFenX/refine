use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolStance,
};

pub struct SolStanceInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SolStanceInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SolStance> for SolStanceInfo {
    fn from(sol_stance: &SolStance) -> Self {
        SolStanceInfo::new(
            sol_stance.get_id(),
            sol_stance.get_fit_id(),
            sol_stance.get_a_item_id(),
            sol_stance.get_bool_state(),
        )
    }
}
