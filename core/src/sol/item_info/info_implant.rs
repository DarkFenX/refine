use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolImplant,
};

pub struct SolImplantInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SolImplantInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SolImplant> for SolImplantInfo {
    fn from(sol_implant: &SolImplant) -> Self {
        SolImplantInfo::new(
            sol_implant.base.id,
            sol_implant.fit_id,
            sol_implant.base.a_item_id,
            sol_implant.get_bool_state(),
        )
    }
}
