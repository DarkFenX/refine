#[derive(serde::Serialize)]
pub(crate) struct HRigInfo {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) enabled: bool,
}
impl From<&rc::SsRigInfo> for HRigInfo {
    fn from(ss_rig_info: &rc::SsRigInfo) -> Self {
        Self {
            id: ss_rig_info.id,
            fit_id: ss_rig_info.fit_id,
            type_id: ss_rig_info.a_item_id,
            enabled: ss_rig_info.enabled,
        }
    }
}
