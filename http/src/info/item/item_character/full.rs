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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HCharacterInfoFull {
    pub(super) fn from_core(core_character: &mut rc::CharacterMut) -> Self {
        Self {
            partial_info: HCharacterInfoPartial::from_core(core_character),
            extended_info: HItemExtendedInfo::from_core(core_character),
        }
    }
}
