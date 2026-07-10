use serde::Serialize;

use super::{
    sw_effect_full::HSwEffectInfoFull, sw_effect_id::HSwEffectInfoId, sw_effect_partial::HSwEffectInfoPartial,
};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HSwEffectInfo {
    Id(HSwEffectInfoId),
    Partial(HSwEffectInfoPartial),
    Full(HSwEffectInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSwEffectInfo {
    pub(in crate::info::item) fn from_core(core_sw_effect: &mut rc::SwEffectMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HSwEffectInfoId::from_core(core_sw_effect)),
            HItemInfoMode::Partial => Self::Partial(HSwEffectInfoPartial::from_core(core_sw_effect)),
            HItemInfoMode::Full => Self::Full(HSwEffectInfoFull::from_core(core_sw_effect)),
        }
    }
}
