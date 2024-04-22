#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HCharacterInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SolCharacterInfo> for HCharacterInfoId {
    fn from(core_character_info: &rc::SolCharacterInfo) -> Self {
        Self {
            id: core_character_info.id,
        }
    }
}
