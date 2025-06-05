use full::HProjEffectInfoFull;
use id::HProjEffectInfoId;
use partial::HProjEffectInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HProjEffectInfo {
    Id(HProjEffectInfoId),
    Partial(HProjEffectInfoPartial),
    Full(HProjEffectInfoFull),
}
impl HProjEffectInfo {
    pub(crate) fn mk_info(core_proj_effect: &mut rc::ProjEffectMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_proj_effect.into()),
            HItemInfoMode::Partial => Self::Partial(core_proj_effect.into()),
            HItemInfoMode::Full => Self::Full(core_proj_effect.into()),
        }
    }
}
