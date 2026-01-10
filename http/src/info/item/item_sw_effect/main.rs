use serde::Serialize;

use super::{full::HSwEffectInfoFull, id::HSwEffectInfoId, partial::HSwEffectInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HSwEffectInfo {
    Id(HSwEffectInfoId),
    Partial(HSwEffectInfoPartial),
    Full(HSwEffectInfoFull),
}
impl HSwEffectInfo {
    pub(in crate::info::item) fn mk_info(core_sw_effect: &mut rc::SwEffectMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_sw_effect.into()),
            HItemInfoMode::Partial => Self::Partial(core_sw_effect.into()),
            HItemInfoMode::Full => Self::Full(core_sw_effect.into()),
        }
    }
}
