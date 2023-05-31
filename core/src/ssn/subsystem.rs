use crate::{
    defs::{ReeId, ReeInt},
    ssi,
};

pub struct SubsystemInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub enabled: bool,
}
impl SubsystemInfo {
    fn new(id: ReeId, fit_id: ReeId, type_id: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            enabled,
        }
    }
}
impl From<&ssi::Subsystem> for SubsystemInfo {
    fn from(s: &ssi::Subsystem) -> Self {
        SubsystemInfo::new(s.id, s.fit_id, s.type_id, s.get_bool_state())
    }
}
