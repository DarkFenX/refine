use crate::sol::{ItemId, ItemTypeId, uad::item::SwEffect};

pub struct SwEffectInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub enabled: bool,
}
impl From<&SwEffect> for SwEffectInfo {
    fn from(sol_sw_effect: &SwEffect) -> Self {
        SwEffectInfo {
            id: sol_sw_effect.get_item_id(),
            type_id: sol_sw_effect.get_a_item_id(),
            enabled: sol_sw_effect.get_sw_effect_state(),
        }
    }
}
