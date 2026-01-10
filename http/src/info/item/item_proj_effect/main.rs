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
impl HProjEffectInfo {
    pub(in crate::info::item) fn mk_info(core_proj_effect: &mut rc::ProjEffectMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_proj_effect.into()),
            HItemInfoMode::Partial => Self::Partial(core_proj_effect.into()),
            HItemInfoMode::Full => Self::Full(core_proj_effect.into()),
        }
    }
}
