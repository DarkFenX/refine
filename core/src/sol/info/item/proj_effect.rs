use crate::{
    defs::{EItemId, SolItemId},
    sol::uad::item::SolProjEffect,
};

pub struct SolProjEffectInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub enabled: bool,
    pub projs: Vec<SolItemId>,
}
impl SolProjEffectInfo {
    fn new(id: SolItemId, type_id: EItemId, enabled: bool, projs: Vec<SolItemId>) -> Self {
        Self {
            id,
            type_id,
            enabled,
            projs,
        }
    }
}
impl From<&SolProjEffect> for SolProjEffectInfo {
    fn from(sol_proj_effect: &SolProjEffect) -> Self {
        SolProjEffectInfo::new(
            sol_proj_effect.get_id(),
            sol_proj_effect.get_type_id(),
            sol_proj_effect.get_bool_state(),
            sol_proj_effect.get_projs().iter_items().copied().collect(),
        )
    }
}
