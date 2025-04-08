use crate::sol::{FitId, ItemId, ItemTypeId, SlotIndex, uad::item::Implant};

pub struct ImplantInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub slot: Option<SlotIndex>,
    pub enabled: bool,
}
impl ImplantInfo {
    pub(in crate::sol) fn from_implant(implant: &Implant) -> Self {
        Self {
            id: implant.get_item_id(),
            type_id: implant.get_a_item_id(),
            fit_id: implant.get_fit_id(),
            slot: implant.get_a_slot(),
            enabled: implant.get_implant_state(),
        }
    }
}
