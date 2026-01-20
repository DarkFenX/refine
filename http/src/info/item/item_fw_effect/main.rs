use serde::Serialize;

use super::{full::HFwEffectInfoFull, id::HFwEffectInfoId, partial::HFwEffectInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HFwEffectInfo {
    Id(HFwEffectInfoId),
    Partial(HFwEffectInfoPartial),
    Full(HFwEffectInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFwEffectInfo {
    pub(in crate::info::item) fn from_core(core_fw_effect: &mut rc::FwEffectMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HFwEffectInfoId::from_core(core_fw_effect)),
            HItemInfoMode::Partial => Self::Partial(HFwEffectInfoPartial::from_core(core_fw_effect)),
            HItemInfoMode::Full => Self::Full(HFwEffectInfoFull::from_core(core_fw_effect)),
        }
    }
}
