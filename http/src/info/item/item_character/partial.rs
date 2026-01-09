use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HCharacterInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    enabled: bool,
}
impl From<&mut rc::CharacterMut<'_>> for HCharacterInfoPartial {
    fn from(core_character: &mut rc::CharacterMut) -> Self {
        Self {
            id: core_character.get_item_id(),
            kind: "character",
            type_id: core_character.get_type_id().into_i32(),
            fit_id: core_character.get_fit().get_fit_id(),
            enabled: core_character.get_state(),
        }
    }
}
