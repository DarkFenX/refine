use crate::info::item::extended::HItemExtendedInfo;

use super::HCharacterInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HCharacterInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HCharacterInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl From<&mut rc::CharacterMut<'_>> for HCharacterInfoFull {
    fn from(core_character: &mut rc::CharacterMut) -> Self {
        Self {
            partial_info: core_character.into(),
            extended_info: core_character.into(),
        }
    }
}
