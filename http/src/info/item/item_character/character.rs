use serde::Serialize;

use super::{
    character_full::HCharacterInfoFull, character_id::HCharacterInfoId, character_partial::HCharacterInfoPartial,
};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HCharacterInfo {
    Id(HCharacterInfoId),
    Partial(HCharacterInfoPartial),
    Full(HCharacterInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HCharacterInfo {
    pub(in crate::info::item) fn from_core(core_character: &mut rc::CharacterMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HCharacterInfoId::from_core(core_character)),
            HItemInfoMode::Partial => Self::Partial(HCharacterInfoPartial::from_core(core_character)),
            HItemInfoMode::Full => Self::Full(HCharacterInfoFull::from_core(core_character)),
        }
    }
}
