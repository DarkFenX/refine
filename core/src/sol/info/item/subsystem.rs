use crate::sol::{FitId, ItemId, ItemTypeId, SlotIndex, uad::item::Subsystem};

pub struct SubsystemInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub slot: Option<SlotIndex>,
    pub enabled: bool,
}
impl From<&Subsystem> for SubsystemInfo {
    fn from(sol_subsystem: &Subsystem) -> Self {
        SubsystemInfo {
            id: sol_subsystem.get_item_id(),
            type_id: sol_subsystem.get_a_item_id(),
            fit_id: sol_subsystem.get_fit_id(),
            slot: sol_subsystem.get_a_slot(),
            enabled: sol_subsystem.get_subsystem_state(),
        }
    }
}
