use crate::{
    defs::{ReeId, ReeInt},
    ss::item::Rig,
};

pub struct RigInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub enabled: bool,
}
impl RigInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, enabled: bool) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            enabled,
        }
    }
}
impl From<&Rig> for RigInfo {
    fn from(r: &Rig) -> Self {
        RigInfo::new(r.item_id, r.fit_id, r.type_id, r.get_bool_state())
    }
}
