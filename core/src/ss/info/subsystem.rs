use crate::{
    defs::{ReeId, ReeInt},
    ss::item::Subsystem,
};

pub struct SubsystemInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub enabled: bool,
}
impl SubsystemInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, enabled: bool) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            enabled,
        }
    }
}
impl From<&Subsystem> for SubsystemInfo {
    fn from(s: &Subsystem) -> Self {
        SubsystemInfo::new(s.item_id, s.fit_id, s.type_id, s.get_bool_state())
    }
}
