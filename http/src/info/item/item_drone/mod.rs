use full::HDroneInfoFull;
use id::HDroneInfoId;
use partial::HDroneInfoPartial;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HDroneInfo {
    Id(HDroneInfoId),
    Partial(HDroneInfoPartial),
    Full(HDroneInfoFull),
}
impl HDroneInfo {
    pub(in crate::info::item) fn mk_info(core_drone: &mut rc::DroneMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_drone.into()),
            HItemInfoMode::Partial => Self::Partial(core_drone.into()),
            HItemInfoMode::Full => Self::Full(core_drone.into()),
        }
    }
}
