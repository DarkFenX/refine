use full::HDroneInfoFull;
use id::HDroneInfoId;
use partial::HDroneInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HDroneInfo {
    Id(HDroneInfoId),
    Partial(HDroneInfoPartial),
    Full(HDroneInfoFull),
}
impl HDroneInfo {
    pub(crate) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_drone_info: &rc::DroneInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_drone_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_drone_info.into()),
            HItemInfoMode::Full => Self::Full(HDroneInfoFull::mk_info(core_sol, core_drone_info)),
        }
    }
}
