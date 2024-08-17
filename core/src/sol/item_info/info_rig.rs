use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolRig,
};

pub struct SolRigInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub type_id: EItemId,
    pub enabled: bool,
}
impl SolRigInfo {
    fn new(id: SolItemId, fit_id: SolFitId, type_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            enabled,
        }
    }
}
impl From<&SolRig> for SolRigInfo {
    fn from(sol_rig: &SolRig) -> Self {
        SolRigInfo::new(
            sol_rig.get_id(),
            sol_rig.get_fit_id(),
            sol_rig.get_type_id(),
            sol_rig.get_bool_state(),
        )
    }
}
