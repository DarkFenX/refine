use full::HBoosterInfoFull;
use id::HBoosterInfoId;
use partial::HBoosterInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HBoosterInfo {
    Id(HBoosterInfoId),
    Partial(HBoosterInfoPartial),
    Full(HBoosterInfoFull),
}
impl HBoosterInfo {
    pub(crate) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_booster_info: &rc::SsBoosterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_booster_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_booster_info.into()),
            HItemInfoMode::Full => Self::Full(HBoosterInfoFull::mk_info(core_ss, core_booster_info)),
        }
    }
}
