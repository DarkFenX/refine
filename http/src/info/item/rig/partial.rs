#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HRigInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    pub(crate) enabled: bool,
}
impl From<&rc::RigInfo> for HRigInfoPartial {
    fn from(core_rig_info: &rc::RigInfo) -> Self {
        Self {
            id: core_rig_info.id,
            kind: "rig",
            type_id: core_rig_info.type_id,
            fit_id: core_rig_info.fit_id,
            enabled: core_rig_info.enabled,
        }
    }
}
