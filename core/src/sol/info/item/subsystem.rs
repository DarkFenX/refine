use crate::sol::{
    FitId, ItemId, ItemTypeId, SlotIndex,
    uad::{Uad, item::Subsystem},
};

pub struct SubsystemInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub slot: Option<SlotIndex>,
    pub enabled: bool,
}
impl SubsystemInfo {
    pub(in crate::sol) fn from_subsystem(uad: &Uad, subsystem: &Subsystem) -> Self {
        Self {
            id: subsystem.get_item_id(),
            type_id: subsystem.get_a_item_id(),
            fit_id: uad.fits.id_by_key(subsystem.get_fit_key()),
            slot: subsystem.get_a_slot(),
            enabled: subsystem.get_subsystem_state(),
        }
    }
}
