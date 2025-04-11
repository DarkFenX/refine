use crate::sol::{
    FitId, ItemId, ItemTypeId,
    uad::{Uad, item::UadCharge},
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
    pub(in crate::sol) fn from_charge(uad: &Uad, charge: &UadCharge) -> Self {
        Self {
            id: charge.get_item_id(),
            type_id: charge.get_a_item_id(),
            fit_id: uad.fits.id_by_key(charge.get_fit_key()),
            cont_item_id: uad.items.id_by_key(charge.get_cont_item_key()),
            enabled: !charge.get_force_disable(),
        }
    }
}
