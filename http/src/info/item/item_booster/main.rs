use serde::Serialize;

use super::{full::HBoosterInfoFull, id::HBoosterInfoId, partial::HBoosterInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HBoosterInfo {
    Id(HBoosterInfoId),
    Partial(HBoosterInfoPartial),
    Full(HBoosterInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HBoosterInfo {
    pub(in crate::info::item) fn from_core(core_booster: &mut rc::BoosterMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HBoosterInfoId::from_core(core_booster)),
            HItemInfoMode::Partial => Self::Partial(HBoosterInfoPartial::from_core(core_booster)),
            HItemInfoMode::Full => Self::Full(HBoosterInfoFull::from_core(core_booster)),
        }
    }
}
