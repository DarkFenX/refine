use full::HFleetInfoFull;
use id::HFleetInfoId;

use crate::{info::HFleetInfoMode, util::HResult};

mod full;
mod id;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HFleetInfo {
    Id(HFleetInfoId),
    Full(HFleetInfoFull),
}
impl HFleetInfo {
    pub(crate) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        fleet_id: &rc::SolFleetId,
        fleet_mode: HFleetInfoMode,
    ) -> HResult<Self> {
        let info = match fleet_mode {
            HFleetInfoMode::Id => Self::Id(HFleetInfoId::mk_info(core_sol, fleet_id)?),
            HFleetInfoMode::Full => Self::Full(HFleetInfoFull::mk_info(core_sol, fleet_id)?),
        };
        Ok(info)
    }
}
