use serde::Serialize;

use super::{full::HDroneInfoFull, id::HDroneInfoId, partial::HDroneInfoPartial};
use crate::info::HItemInfoMode;

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
