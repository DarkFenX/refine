use crate::sol::{FitId, ItemId, ItemTypeId, uad::item::FwEffect};

pub struct FwEffectInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub enabled: bool,
}
impl FwEffectInfo {
    pub(in crate::sol) fn from_fw_effect(fw_effect: &FwEffect) -> Self {
        Self {
            id: fw_effect.get_item_id(),
            type_id: fw_effect.get_a_item_id(),
            fit_id: fw_effect.get_fit_id(),
            enabled: fw_effect.get_fw_effect_state(),
        }
    }
}
