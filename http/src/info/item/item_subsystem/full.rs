use serde::Serialize;

use super::HSubsystemInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HSubsystemInfoFull {
    #[serde(flatten)]
    partial_info: HSubsystemInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}
impl From<&mut rc::SubsystemMut<'_>> for HSubsystemInfoFull {
    fn from(core_subsystem: &mut rc::SubsystemMut) -> Self {
        Self {
            partial_info: core_subsystem.into(),
            extended_info: core_subsystem.into(),
        }
    }
}
