use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolRig,
};

pub struct SolRigInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SolRigInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SolRig> for SolRigInfo {
    fn from(sol_rig: &SolRig) -> Self {
        SolRigInfo::new(sol_rig.id, sol_rig.fit_id, sol_rig.a_item_id, sol_rig.get_bool_state())
    }
}
