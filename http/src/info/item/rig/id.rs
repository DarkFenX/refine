#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HRigInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::RigInfo> for HRigInfoId {
    fn from(core_rig_info: &rc::RigInfo) -> Self {
        Self { id: core_rig_info.id }
    }
}
