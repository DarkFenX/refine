use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HCharacterInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&mut rc::CharacterMut<'_>> for HCharacterInfoId {
    fn from(core_character: &mut rc::CharacterMut) -> Self {
        Self {
            id: core_character.get_item_id(),
        }
    }
}
