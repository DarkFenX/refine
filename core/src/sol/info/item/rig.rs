use crate::sol::{
    FitId, ItemId, ItemTypeId,
    uad::{Uad, item::Rig},
};

pub struct RigInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub enabled: bool,
}
impl RigInfo {
    pub(in crate::sol) fn from_rig(uad: &Uad, rig: &Rig) -> Self {
        Self {
            id: rig.get_item_id(),
            type_id: rig.get_a_item_id(),
            fit_id: uad.fits.id_by_key(rig.get_fit_key()),
            enabled: rig.get_rig_state(),
        }
    }
}
