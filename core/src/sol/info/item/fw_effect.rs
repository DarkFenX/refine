use crate::sol::{FitId, ItemId, ItemTypeId, uad::item::FwEffect};

pub struct FwEffectInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub enabled: bool,
}
impl From<&FwEffect> for FwEffectInfo {
    fn from(sol_fw_effect: &FwEffect) -> Self {
        FwEffectInfo {
            id: sol_fw_effect.get_item_id(),
            type_id: sol_fw_effect.get_a_item_id(),
            fit_id: sol_fw_effect.get_fit_id(),
            enabled: sol_fw_effect.get_fw_effect_state(),
        }
    }
}
