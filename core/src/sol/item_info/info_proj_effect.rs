use crate::{
    defs::{EItemId, SolItemId},
    sol::item::SolProjEffect,
};

pub struct SolProjEffectInfo {
    pub id: SolItemId,
    pub a_item_id: EItemId,
    pub enabled: bool,
    pub projs: Vec<SolItemId>,
}
impl SolProjEffectInfo {
    fn new(id: SolItemId, a_item_id: EItemId, enabled: bool, projs: Vec<SolItemId>) -> Self {
        Self {
            id,
            a_item_id,
            enabled,
            projs,
        }
    }
}
impl From<&SolProjEffect> for SolProjEffectInfo {
    fn from(sol_proj_effect: &SolProjEffect) -> Self {
        SolProjEffectInfo::new(
            sol_proj_effect.base.id,
            sol_proj_effect.base.a_item_id,
            sol_proj_effect.get_bool_state(),
            sol_proj_effect.projs.iter_items().map(|v| *v).collect(),
        )
    }
}
