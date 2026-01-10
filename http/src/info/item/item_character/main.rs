use serde::Serialize;

use super::{full::HCharacterInfoFull, id::HCharacterInfoId, partial::HCharacterInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HCharacterInfo {
    Id(HCharacterInfoId),
    Partial(HCharacterInfoPartial),
    Full(HCharacterInfoFull),
}
impl HCharacterInfo {
    pub(in crate::info::item) fn mk_info(core_character: &mut rc::CharacterMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_character.into()),
            HItemInfoMode::Partial => Self::Partial(core_character.into()),
            HItemInfoMode::Full => Self::Full(core_character.into()),
        }
    }
}
