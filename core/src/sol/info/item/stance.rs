use crate::sol::{FitId, ItemId, ItemTypeId, uad::item::Stance};

pub struct StanceInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub enabled: bool,
}
impl StanceInfo {
    pub(in crate::sol) fn from_stance(stance: &Stance) -> Self {
        Self {
            id: stance.get_item_id(),
            type_id: stance.get_a_item_id(),
            fit_id: stance.get_fit_id(),
            enabled: stance.get_stance_state(),
        }
    }
}
