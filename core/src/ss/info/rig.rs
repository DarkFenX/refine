use crate::{
    defs::{ItemId, SsFitId, SsItemId},
    ss::item::SsRig,
};

pub struct SsRigInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: ItemId,
    pub enabled: bool,
}
impl SsRigInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: ItemId, enabled: bool) -> Self {
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
