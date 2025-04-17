use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HCharacterInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    pub(crate) enabled: bool,
}
impl From<&mut rc::CharacterMut<'_>> for HCharacterInfoPartial {
    fn from(core_character: &mut rc::CharacterMut) -> Self {
        Self {
            id: core_character.get_item_id(),
            kind: "character",
            type_id: core_character.get_type_id(),
            fit_id: core_character.get_fit().get_fit_id(),
            enabled: core_character.get_state(),
        }
    }
}
