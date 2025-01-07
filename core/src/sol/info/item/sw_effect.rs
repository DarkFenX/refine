use crate::{
    defs::{EItemId, SolItemId},
    sol::uad::item::SolSwEffect,
};

pub struct SolSwEffectInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub enabled: bool,
}
impl SolSwEffectInfo {
    fn new(id: SolItemId, type_id: EItemId, enabled: bool) -> Self {
        Self { id, type_id, enabled }
    }
}
impl From<&SolSwEffect> for SolSwEffectInfo {
    fn from(sol_sw_effect: &SolSwEffect) -> Self {
        SolSwEffectInfo::new(
            sol_sw_effect.get_id(),
            sol_sw_effect.get_type_id(),
            sol_sw_effect.get_bool_state(),
        )
    }
}
