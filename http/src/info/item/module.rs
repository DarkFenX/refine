#[derive(serde::Serialize)]
pub(crate) struct ModuleInfo {
    #[serde(with = "crate::util::serde_string")]
    pub id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub fit_id: rc::ReeId,
    pub type_id: rc::ReeInt,
}
impl From<&rc::SsModuleInfo> for ModuleInfo {
    fn from(value: &rc::SsModuleInfo) -> Self {
        Self {
            id: value.id,
            fit_id: value.fit_id,
            type_id: value.a_item_id,
        }
    }
}
