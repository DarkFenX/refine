use crate::{
    defs::{EItemId, SlotNumber, SolFitId, SolItemId},
    sol::uad::item::SolImplant,
};

pub struct SolImplantInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub slot: Option<SlotNumber>,
    pub enabled: bool,
}
impl SolImplantInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, slot: Option<SlotNumber>, enabled: bool) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            slot,
            enabled,
        }
    }
}
impl From<&SolImplant> for SolImplantInfo {
    fn from(sol_implant: &SolImplant) -> Self {
        SolImplantInfo::new(
            sol_implant.get_id(),
            sol_implant.get_type_id(),
            sol_implant.get_fit_id(),
            sol_implant.get_slot(),
            sol_implant.get_bool_state(),
        )
    }
}
