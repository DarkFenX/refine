use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolStance,
};

pub struct SolStanceInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub enabled: bool,
}
impl SolStanceInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, enabled: bool) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            enabled,
        }
    }
}
impl From<&SolStance> for SolStanceInfo {
    fn from(sol_stance: &SolStance) -> Self {
        SolStanceInfo::new(
            sol_stance.get_id(),
            sol_stance.get_type_id(),
            sol_stance.get_fit_id(),
            sol_stance.get_bool_state(),
        )
    }
}
