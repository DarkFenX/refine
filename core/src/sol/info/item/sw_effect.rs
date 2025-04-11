use crate::sol::{ItemId, ItemTypeId, uad::item::UadSwEffect};

pub struct SwEffectInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub enabled: bool,
}
impl SwEffectInfo {
    pub(in crate::sol) fn from_sw_effect(sw_effect: &UadSwEffect) -> Self {
        Self {
            id: sw_effect.get_item_id(),
            type_id: sw_effect.get_a_item_id(),
            enabled: sw_effect.get_sw_effect_state(),
        }
    }
}
