use crate::{
    defs::{ItemId, SsFitId, SsItemId},
    ss::item::SsSubsystem,
};

pub struct SsSubsystemInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: ItemId,
    pub enabled: bool,
}
impl SsSubsystemInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: ItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SsSubsystem> for SsSubsystemInfo {
    fn from(ss_subsystem: &SsSubsystem) -> Self {
        SsSubsystemInfo::new(
            ss_subsystem.id,
            ss_subsystem.fit_id,
            ss_subsystem.a_item_id,
            ss_subsystem.get_bool_state(),
        )
    }
}
