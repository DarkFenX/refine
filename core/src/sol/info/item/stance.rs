use crate::sol::{FitId, ItemId, ItemTypeId, uad::item::Stance};

pub struct StanceInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub enabled: bool,
}
impl From<&Stance> for StanceInfo {
    fn from(sol_stance: &Stance) -> Self {
        StanceInfo {
            id: sol_stance.get_item_id(),
            type_id: sol_stance.get_a_item_id(),
            fit_id: sol_stance.get_fit_id(),
            enabled: sol_stance.get_stance_state(),
        }
    }
}
