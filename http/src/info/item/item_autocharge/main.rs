use serde::Serialize;

use super::{full::HAutochargeInfoFull, id::HAutochargeInfoId, partial::HAutochargeInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HAutochargeInfo {
    Id(HAutochargeInfoId),
    Partial(HAutochargeInfoPartial),
    Full(HAutochargeInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HAutochargeInfo {
    pub(in crate::info::item) fn from_core(core_autocharge: &mut rc::AutochargeMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HAutochargeInfoId::from_core(core_autocharge)),
            HItemInfoMode::Partial => Self::Partial(HAutochargeInfoPartial::from_core(core_autocharge)),
            HItemInfoMode::Full => Self::Full(HAutochargeInfoFull::from_core(core_autocharge)),
        }
    }
}
