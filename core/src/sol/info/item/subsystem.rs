use crate::sol::{FitId, ItemId, ItemTypeId, SlotIndex, uad::item::Subsystem};

pub struct SubsystemInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub slot: Option<SlotIndex>,
    pub enabled: bool,
}
impl SubsystemInfo {
    pub(in crate::sol) fn from_subsystem(subsystem: &Subsystem) -> Self {
        Self {
            id: subsystem.get_item_id(),
            type_id: subsystem.get_a_item_id(),
            fit_id: subsystem.get_fit_id(),
            slot: subsystem.get_a_slot(),
            enabled: subsystem.get_subsystem_state(),
        }
    }
}
