use crate::{
    defs::{ReeId, ReeInt},
    ssi,
};

pub struct SsSubsystemInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub a_item_id: ReeInt,
    pub enabled: bool,
}
impl SsSubsystemInfo {
    fn new(id: ReeId, fit_id: ReeId, a_item_id: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&ssi::SsSubsystem> for SsSubsystemInfo {
    fn from(ss_subsystem: &ssi::SsSubsystem) -> Self {
        SsSubsystemInfo::new(
            ss_subsystem.id,
            ss_subsystem.fit_id,
            ss_subsystem.a_item_id,
            ss_subsystem.get_bool_state(),
        )
    }
}
