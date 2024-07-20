use crate::{
    defs::{EItemId, SolItemId},
    sol::item::SolSwEffect,
};

pub struct SolSwEffectInfo {
    pub id: SolItemId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SolSwEffectInfo {
    fn new(id: SolItemId, a_item_id: EItemId, enabled: bool) -> Self {
        Self { id, a_item_id, enabled }
    }
}
impl From<&SolSwEffect> for SolSwEffectInfo {
    fn from(sol_sw_effect: &SolSwEffect) -> Self {
        SolSwEffectInfo::new(
            sol_sw_effect.base.id,
            sol_sw_effect.base.a_item_id,
            sol_sw_effect.get_bool_state(),
        )
    }
}
