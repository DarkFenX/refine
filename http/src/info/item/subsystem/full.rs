use crate::info::item::extended::HItemExtendedInfo;

use super::HSubsystemInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HSubsystemInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HSubsystemInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl From<&mut rc::SubsystemMut<'_>> for HSubsystemInfoFull {
    fn from(core_subsystem: &mut rc::SubsystemMut) -> Self {
        Self {
            partial_info: core_subsystem.into(),
            extended_info: core_subsystem.into(),
        }
    }
}
