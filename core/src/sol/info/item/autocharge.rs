use crate::sol::{FitId, ItemId, ItemTypeId, uad::item::Autocharge};

pub struct AutochargeInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub cont_id: ItemId,
    pub enabled: bool,
    // No projections because they fully match to projections of parent item
}
impl From<&Autocharge> for AutochargeInfo {
    fn from(sol_autocharge: &Autocharge) -> Self {
        AutochargeInfo {
            id: sol_autocharge.get_item_id(),
            type_id: sol_autocharge.get_a_item_id(),
            fit_id: sol_autocharge.get_fit_id(),
            cont_id: sol_autocharge.get_cont_item_id(),
            enabled: !sol_autocharge.get_force_disable(),
        }
    }
}
