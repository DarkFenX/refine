use crate::sol::{FitId, ItemId, ItemTypeId, uad::item::Rig};

pub struct RigInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub enabled: bool,
}
impl From<&Rig> for RigInfo {
    fn from(sol_rig: &Rig) -> Self {
        Self {
            id: sol_rig.get_item_id(),
            type_id: sol_rig.get_a_item_id(),
            fit_id: sol_rig.get_fit_id(),
            enabled: sol_rig.get_rig_state(),
        }
    }
}
