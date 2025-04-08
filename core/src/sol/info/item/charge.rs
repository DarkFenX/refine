use crate::sol::{
    FitId, ItemId, ItemTypeId,
    uad::{Uad, item::Charge},
};

pub struct ChargeInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub cont_item_id: ItemId,
    pub enabled: bool,
    // No projections because they fully match to projections of parent item
}
impl ChargeInfo {
    pub(in crate::sol) fn from_charge(uad: &Uad, charge: &Charge) -> Self {
        Self {
            id: charge.get_item_id(),
            type_id: charge.get_a_item_id(),
            fit_id: charge.get_fit_id(),
            cont_item_id: uad.items.id_by_key(charge.get_cont_item_key()),
            enabled: !charge.get_force_disable(),
        }
    }
}
