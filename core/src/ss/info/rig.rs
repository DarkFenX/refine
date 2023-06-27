use crate::{
    defs::{ReeId, ReeInt},
    ss::item::SsRig,
};

pub struct SsRigInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub a_item_id: ReeInt,
    pub enabled: bool,
}
impl SsRigInfo {
    fn new(id: ReeId, fit_id: ReeId, a_item_id: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SsRig> for SsRigInfo {
    fn from(ss_rig: &SsRig) -> Self {
        SsRigInfo::new(ss_rig.id, ss_rig.fit_id, ss_rig.a_item_id, ss_rig.get_bool_state())
    }
}
