use serde::Serialize;

use super::partial::HDroneInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HDroneInfoFull {
    #[serde(flatten)]
    partial_info: HDroneInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HDroneInfoFull {
    pub(super) fn from_core(core_drone: &mut rc::DroneMut) -> Self {
        Self {
            partial_info: HDroneInfoPartial::from_core(core_drone),
            extended_info: HItemExtendedInfo::from_core(core_drone),
        }
    }
}
