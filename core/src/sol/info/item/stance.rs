use crate::sol::{
    FitId, ItemId, ItemTypeId,
    uad::{Uad, item::Stance},
};

pub struct StanceInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub enabled: bool,
}
impl StanceInfo {
    pub(in crate::sol) fn from_stance(uad: &Uad, stance: &Stance) -> Self {
        Self {
            id: stance.get_item_id(),
            type_id: stance.get_a_item_id(),
            fit_id: uad.fits.id_by_key(stance.get_fit_key()),
            enabled: stance.get_stance_state(),
        }
    }
}
