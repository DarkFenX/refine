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
    pub(crate) fn mk_info(core_drone: &mut rc::DroneMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_drone.into()),
            HItemInfoMode::Partial => Self::Partial(core_drone.into()),
            HItemInfoMode::Full => Self::Full(core_drone.into()),
        }
    }
}
