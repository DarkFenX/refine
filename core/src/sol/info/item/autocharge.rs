use crate::sol::{EffectId, FitId, ItemId, ItemTypeId, uad::item::Autocharge};

pub struct AutochargeInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub cont_item_id: ItemId,
    pub cont_effect_id: EffectId,
    pub enabled: bool,
    // No projections because they fully match to projections of parent item
}
impl From<&Autocharge> for AutochargeInfo {
    fn from(sol_autocharge: &Autocharge) -> Self {
        Self {
            id: sol_autocharge.get_item_id(),
            type_id: sol_autocharge.get_a_item_id(),
            fit_id: sol_autocharge.get_fit_id(),
            cont_item_id: sol_autocharge.get_cont_item_id(),
            cont_effect_id: sol_autocharge.get_cont_effect_id().into(),
            enabled: !sol_autocharge.get_force_disable(),
        }
    }
}
