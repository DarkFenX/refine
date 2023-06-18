#[derive(serde::Serialize)]
pub(crate) struct HCharacterInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
}
impl From<&rc::SsCharacterInfo> for HCharacterInfoId {
    fn from(core_character_info: &rc::SsCharacterInfo) -> Self {
        Self {
            id: core_character_info.id,
        }
    }
}
