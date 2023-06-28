#[derive(serde::Serialize)]
pub(crate) struct HRigInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsRigInfo> for HRigInfoId {
    fn from(core_rig_info: &rc::SsRigInfo) -> Self {
        Self { id: core_rig_info.id }
    }
}
