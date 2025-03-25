#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HCharacterInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&rc::CharacterInfo> for HCharacterInfoId {
    fn from(core_character_info: &rc::CharacterInfo) -> Self {
        Self {
            id: core_character_info.id,
        }
    }
}
