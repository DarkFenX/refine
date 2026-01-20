use serde::Serialize;

use super::partial::HSubsystemInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HSubsystemInfoFull {
    #[serde(flatten)]
    partial_info: HSubsystemInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSubsystemInfoFull {
    pub(super) fn from_core(core_subsystem: &mut rc::SubsystemMut) -> Self {
        Self {
            partial_info: HSubsystemInfoPartial::from_core(core_subsystem),
            extended_info: HItemExtendedInfo::from_core(core_subsystem),
        }
    }
}
