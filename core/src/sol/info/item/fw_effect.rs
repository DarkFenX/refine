use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::uad::item::SolFwEffect,
};

pub struct SolFwEffectInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub enabled: bool,
}
impl SolFwEffectInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, enabled: bool) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            enabled,
        }
    }
}
impl From<&SolFwEffect> for SolFwEffectInfo {
    fn from(sol_fw_effect: &SolFwEffect) -> Self {
        SolFwEffectInfo::new(
            sol_fw_effect.get_id(),
            sol_fw_effect.get_type_id(),
            sol_fw_effect.get_fit_id(),
            sol_fw_effect.get_fw_effect_state(),
        )
    }
}
