use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HCharacterInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HCharacterInfoId {
    pub(super) fn from_core(core_character: &mut rc::CharacterMut) -> Self {
        Self {
            id: core_character.get_item_id(),
        }
    }
}
