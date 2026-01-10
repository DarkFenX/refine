use serde::Serialize;

use super::partial::HCharacterInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HCharacterInfoFull {
    #[serde(flatten)]
    partial_info: HCharacterInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}
impl From<&mut rc::CharacterMut<'_>> for HCharacterInfoFull {
    fn from(core_character: &mut rc::CharacterMut) -> Self {
        Self {
            partial_info: core_character.into(),
            extended_info: core_character.into(),
        }
    }
}
