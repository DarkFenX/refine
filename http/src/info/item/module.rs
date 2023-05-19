#[derive(serde::Serialize)]
pub(crate) struct ModuleInfo {
    pub item_id: String,
    pub fit_id: String,
    pub type_id: reefast::ReeInt,
}
impl From<&reefast::ModuleInfo> for ModuleInfo {
    fn from(value: &reefast::ModuleInfo) -> Self {
        Self {
            item_id: value.item_id.to_string(),
            fit_id: value.fit_id.to_string(),
            type_id: value.type_id,
        }
    }
}
