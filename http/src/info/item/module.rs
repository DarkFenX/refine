use crate::shared::{HState, HModRack};

#[derive(serde::Serialize)]
pub(crate) struct HModuleInfo {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: HState,
    pub(crate) rack: HModRack,
    pub(crate) pos: rc::ReeIdx,
}
impl From<&rc::SsModuleInfo> for HModuleInfo {
    fn from(ss_module_info: &rc::SsModuleInfo) -> Self {
        Self {
            id: ss_module_info.id,
            fit_id: ss_module_info.fit_id,
            type_id: ss_module_info.a_item_id,
            state: (&ss_module_info.state).into(),
            rack: (&ss_module_info.rack).into(),
            pos: ss_module_info.pos,
        }
    }
}
