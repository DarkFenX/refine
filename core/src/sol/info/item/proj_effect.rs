use crate::sol::{
    ItemId, ItemTypeId,
    uad::{Uad, item::UadProjEffect},
};

pub struct ProjEffectInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub enabled: bool,
    pub projs: Vec<ItemId>,
}
impl ProjEffectInfo {
    pub(in crate::sol) fn from_proj_effect(uad: &Uad, proj_effect: &UadProjEffect) -> Self {
        Self {
            id: proj_effect.get_item_id(),
            type_id: proj_effect.get_a_item_id(),
            enabled: proj_effect.get_proj_effect_state(),
            projs: proj_effect
                .get_projs()
                .iter_projectee_item_keys()
                .map(|&projectee_item_key| uad.items.id_by_key(projectee_item_key))
                .collect(),
        }
    }
}
