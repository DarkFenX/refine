use serde::Serialize;

use super::{full::HFleetInfoFull, id::HFleetInfoId};
use crate::info::HFleetInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HFleetInfo {
    Id(HFleetInfoId),
    Full(HFleetInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFleetInfo {
    pub(crate) fn from_core_fleet(core_fleet: &mut rc::FleetMut, fleet_mode: HFleetInfoMode) -> Self {
        match fleet_mode {
            HFleetInfoMode::Id => Self::Id(HFleetInfoId::from_core_fleet(core_fleet)),
            HFleetInfoMode::Full => Self::Full(HFleetInfoFull::from_core(core_fleet)),
        }
    }
}
