use full::HCharacterInfoFull;
use id::HCharacterInfoId;
use partial::HCharacterInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HCharacterInfo {
    Id(HCharacterInfoId),
    Partial(HCharacterInfoPartial),
    Full(HCharacterInfoFull),
}
impl HCharacterInfo {
    pub(crate) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_character_info: &rc::SolCharacterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_character_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_character_info.into()),
            HItemInfoMode::Full => Self::Full(HCharacterInfoFull::mk_info(core_sol, core_character_info)),
        }
    }
}
