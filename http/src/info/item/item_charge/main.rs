use serde::Serialize;

use super::{full::HChargeInfoFull, id::HChargeInfoId, partial::HChargeInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HChargeInfo {
    Id(HChargeInfoId),
    Partial(HChargeInfoPartial),
    Full(HChargeInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HChargeInfo {
    pub(in crate::info::item) fn from_core(core_charge: &mut rc::ChargeMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HChargeInfoId::from_core(core_charge)),
            HItemInfoMode::Partial => Self::Partial(HChargeInfoPartial::from_core(core_charge)),
            HItemInfoMode::Full => Self::Full(HChargeInfoFull::from_core(core_charge)),
        }
    }
}
