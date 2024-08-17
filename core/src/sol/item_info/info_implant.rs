use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolImplant,
};

pub struct SolImplantInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub type_id: EItemId,
    pub enabled: bool,
}
impl SolImplantInfo {
    fn new(id: SolItemId, fit_id: SolFitId, type_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            enabled,
        }
    }
}
impl From<&SolImplant> for SolImplantInfo {
    fn from(sol_implant: &SolImplant) -> Self {
        SolImplantInfo::new(
            sol_implant.get_id(),
            sol_implant.get_fit_id(),
            sol_implant.get_type_id(),
            sol_implant.get_bool_state(),
        )
    }
}
