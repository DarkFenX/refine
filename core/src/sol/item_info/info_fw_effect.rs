use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolFwEffect,
};

pub struct SolFwEffectInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SolFwEffectInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SolFwEffect> for SolFwEffectInfo {
    fn from(sol_fw_effect: &SolFwEffect) -> Self {
        SolFwEffectInfo::new(
            sol_fw_effect.get_id(),
            sol_fw_effect.get_fit_id(),
            sol_fw_effect.get_a_item_id(),
            sol_fw_effect.get_bool_state(),
        )
    }
}
