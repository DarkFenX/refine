use serde::Serialize;

use super::{full::HProjEffectInfoFull, id::HProjEffectInfoId, partial::HProjEffectInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HProjEffectInfo {
    Id(HProjEffectInfoId),
    Partial(HProjEffectInfoPartial),
    Full(HProjEffectInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HProjEffectInfo {
    pub(in crate::info::item) fn from_core(core_proj_effect: &mut rc::ProjEffectMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HProjEffectInfoId::from_core(core_proj_effect)),
            HItemInfoMode::Partial => Self::Partial(HProjEffectInfoPartial::from_core(core_proj_effect)),
            HItemInfoMode::Full => Self::Full(HProjEffectInfoFull::from_core(core_proj_effect)),
        }
    }
}
