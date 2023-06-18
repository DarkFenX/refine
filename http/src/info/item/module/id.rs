#[derive(serde::Serialize)]
pub(crate) struct HModuleInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(skip_serializing_if = "Option::is_none", with = "crate::util::serde_string_opt")]
    pub(crate) charge_id: Option<rc::ReeId>,
}
impl From<&rc::SsModuleInfo> for HModuleInfoId {
    fn from(core_module_info: &rc::SsModuleInfo) -> Self {
        Self {
            id: core_module_info.id,
            charge_id: core_module_info.ss_charge_info.as_ref().map(|v| v.id),
        }
    }
}
