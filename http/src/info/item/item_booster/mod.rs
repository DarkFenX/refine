use full::HBoosterInfoFull;
use id::HBoosterInfoId;
use partial::HBoosterInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;
mod side_effect;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HBoosterInfo {
    Id(HBoosterInfoId),
    Partial(HBoosterInfoPartial),
    Full(HBoosterInfoFull),
}
impl HBoosterInfo {
    pub(crate) fn mk_info(core_booster: &mut rc::BoosterMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_booster.into()),
            HItemInfoMode::Partial => Self::Partial(core_booster.into()),
            HItemInfoMode::Full => Self::Full(core_booster.into()),
        }
    }
}
