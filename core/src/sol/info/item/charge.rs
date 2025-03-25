use crate::sol::{FitId, ItemId, ItemTypeId, uad::item::Charge};

pub struct ChargeInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub cont_id: ItemId,
    pub enabled: bool,
    // No projections because they fully match to projections of parent item
}
impl From<&Charge> for ChargeInfo {
    fn from(sol_charge: &Charge) -> Self {
        ChargeInfo {
            id: sol_charge.get_item_id(),
            type_id: sol_charge.get_a_item_id(),
            fit_id: sol_charge.get_fit_id(),
            cont_id: sol_charge.get_cont_item_id(),
            enabled: !sol_charge.get_force_disable(),
        }
    }
}
