#[derive(serde::Serialize)]
pub(crate) struct HCharacterInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
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
