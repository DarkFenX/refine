#[derive(serde::Serialize)]
pub(crate) struct HCharacterInfo {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) enabled: bool,
}
impl From<&rc::SsCharacterInfo> for HCharacterInfo {
    fn from(ss_char_info: &rc::SsCharacterInfo) -> Self {
        Self {
            id: ss_char_info.id,
            fit_id: ss_char_info.fit_id,
            type_id: ss_char_info.a_item_id,
            enabled: ss_char_info.enabled,
        }
    }
}
