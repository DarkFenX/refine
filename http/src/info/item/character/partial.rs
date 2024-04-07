#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HCharacterInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SsFitId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::SsCharacterInfo> for HCharacterInfoPartial {
    fn from(core_character_info: &rc::SsCharacterInfo) -> Self {
        Self {
            id: core_character_info.id,
            fit_id: core_character_info.fit_id,
            type_id: core_character_info.a_item_id,
            enabled: core_character_info.enabled,
        }
    }
}
