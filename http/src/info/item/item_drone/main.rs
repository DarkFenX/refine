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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HDroneInfo {
    pub(in crate::info::item) fn from_core(core_drone: &mut rc::DroneMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HDroneInfoId::from_core(core_drone)),
            HItemInfoMode::Partial => Self::Partial(HDroneInfoPartial::from_core(core_drone)),
            HItemInfoMode::Full => Self::Full(HDroneInfoFull::from_core(core_drone)),
        }
    }
}
