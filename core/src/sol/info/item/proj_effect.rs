use crate::sol::{ItemId, ItemTypeId, uad::item::ProjEffect};

pub struct ProjEffectInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub enabled: bool,
    pub projs: Vec<ItemId>,
}
impl From<&ProjEffect> for ProjEffectInfo {
    fn from(sol_proj_effect: &ProjEffect) -> Self {
        Self {
            id: sol_proj_effect.get_item_id(),
            type_id: sol_proj_effect.get_a_item_id(),
            enabled: sol_proj_effect.get_proj_effect_state(),
            projs: sol_proj_effect.get_projs().iter_items().copied().collect(),
        }
    }
}
