#[derive(serde::Serialize)]
pub(crate) struct ImplantInfo {
    #[serde(with = "crate::util::serde_string")]
    pub id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub fit_id: rc::ReeId,
    pub type_id: rc::ReeInt,
    pub enabled: bool,
}
impl From<&rc::SsImplantInfo> for ImplantInfo {
    fn from(value: &rc::SsImplantInfo) -> Self {
        Self {
            id: value.id,
            fit_id: value.fit_id,
            type_id: value.a_item_id,
            enabled: value.enabled,
        }
    }
}
