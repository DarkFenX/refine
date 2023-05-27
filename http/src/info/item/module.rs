#[derive(serde::Serialize)]
pub(crate) struct ModuleInfo {
    #[serde(with = "crate::util::serde_string")]
    pub item_id: reefast::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub fit_id: reefast::ReeId,
    pub type_id: reefast::ReeInt,
}
impl From<&reefast::ModuleInfo> for ModuleInfo {
    fn from(value: &reefast::ModuleInfo) -> Self {
        Self {
            item_id: value.item_id,
            fit_id: value.fit_id,
            type_id: value.type_id,
        }
    }
}
