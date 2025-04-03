use crate::sol::{FitId, ItemId, ItemTypeId, SlotIndex, uad::item::Implant};

pub struct ImplantInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub slot: Option<SlotIndex>,
    pub enabled: bool,
}
impl From<&Implant> for ImplantInfo {
    fn from(sol_implant: &Implant) -> Self {
        Self {
            id: sol_implant.get_item_id(),
            type_id: sol_implant.get_a_item_id(),
            fit_id: sol_implant.get_fit_id(),
            slot: sol_implant.get_a_slot(),
            enabled: sol_implant.get_implant_state(),
        }
    }
}
