#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HRigInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) enabled: bool,
}
impl From<&rc::SolRigInfo> for HRigInfoPartial {
    fn from(core_rig_info: &rc::SolRigInfo) -> Self {
        Self {
            id: core_rig_info.id,
            kind: "rig",
            type_id: core_rig_info.a_item_id,
            fit_id: core_rig_info.fit_id,
            enabled: core_rig_info.enabled,
        }
    }
}
