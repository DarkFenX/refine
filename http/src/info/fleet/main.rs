use serde::Serialize;

use super::{full::HFleetInfoFull, id::HFleetInfoId};
use crate::info::HFleetInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HFleetInfo {
    Id(HFleetInfoId),
    Full(HFleetInfoFull),
}
impl HFleetInfo {
    pub(crate) fn mk_info(core_fleet: &mut rc::FleetMut, fleet_mode: HFleetInfoMode) -> Self {
        match fleet_mode {
            HFleetInfoMode::Id => Self::Id(core_fleet.into()),
            HFleetInfoMode::Full => Self::Full(core_fleet.into()),
        }
    }
}
