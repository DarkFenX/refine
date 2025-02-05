use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::uad::item::SolRig,
};

pub struct SolRigInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub enabled: bool,
}
impl SolRigInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, enabled: bool) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            enabled,
        }
    }
}
impl From<&SolRig> for SolRigInfo {
    fn from(sol_rig: &SolRig) -> Self {
        SolRigInfo::new(
            sol_rig.get_id(),
            sol_rig.get_type_id(),
            sol_rig.get_fit_id(),
            sol_rig.get_rig_state(),
        )
    }
}
