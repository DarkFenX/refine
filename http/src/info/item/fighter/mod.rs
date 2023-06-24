use full::HFighterInfoFull;
use id::HFighterInfoId;
use partial::HFighterInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HFighterInfo {
    Id(HFighterInfoId),
    Partial(HFighterInfoPartial),
    Full(HFighterInfoFull),
}
impl HFighterInfo {
    pub(crate) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_fighter_info: &rc::SsFighterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_fighter_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_fighter_info.into()),
            HItemInfoMode::Full => Self::Full(HFighterInfoFull::mk_info(core_ss, core_fighter_info)),
        }
    }
}
