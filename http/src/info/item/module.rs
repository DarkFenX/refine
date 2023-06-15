use crate::shared::HState;

#[derive(serde::Serialize)]
pub(crate) struct HModuleInfo {
    #[serde(with = "crate::util::serde_string")]
    pub id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub fit_id: rc::ReeId,
    pub type_id: rc::ReeInt,
    pub state: HState,
}
impl From<&rc::SsModuleInfo> for HModuleInfo {
    fn from(ss_module_info: &rc::SsModuleInfo) -> Self {
        Self {
            id: ss_module_info.id,
            fit_id: ss_module_info.fit_id,
            type_id: ss_module_info.a_item_id,
            state: ss_module_info.state.into(),
        }
    }
}
