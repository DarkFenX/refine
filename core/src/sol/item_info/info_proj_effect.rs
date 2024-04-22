use crate::{
    defs::{EItemId, SolItemId},
    sol::item::SolProjEffect,
};

pub struct SolProjEffectInfo {
    pub id: SolItemId,
    pub a_item_id: EItemId,
    pub enabled: bool,
    pub tgts: Vec<SolItemId>,
}
impl SolProjEffectInfo {
    fn new(id: SolItemId, a_item_id: EItemId, enabled: bool, tgts: Vec<SolItemId>) -> Self {
        Self {
            id,
            a_item_id,
            enabled,
            tgts,
        }
    }
}
impl From<&SolProjEffect> for SolProjEffectInfo {
    fn from(sol_proj_effect: &SolProjEffect) -> Self {
        SolProjEffectInfo::new(
            sol_proj_effect.id,
            sol_proj_effect.a_item_id,
            sol_proj_effect.get_bool_state(),
            sol_proj_effect.tgts.iter_tgts().map(|v| *v).collect(),
        )
    }
}
