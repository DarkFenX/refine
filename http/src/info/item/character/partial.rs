#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HCharacterInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) enabled: bool,
}
impl From<&rc::CharacterInfo> for HCharacterInfoPartial {
    fn from(core_character_info: &rc::CharacterInfo) -> Self {
        Self {
            id: core_character_info.id,
            kind: "character",
            type_id: core_character_info.type_id,
            fit_id: core_character_info.fit_id,
            enabled: core_character_info.enabled,
        }
    }
}
