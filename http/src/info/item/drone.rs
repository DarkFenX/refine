use crate::shared::HState;

#[derive(serde::Serialize)]
pub(crate) struct HDroneInfo {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: HState,
}
impl From<&rc::SsDroneInfo> for HDroneInfo {
    fn from(ss_drone_info: &rc::SsDroneInfo) -> Self {
        Self {
            id: ss_drone_info.id,
            fit_id: ss_drone_info.fit_id,
            type_id: ss_drone_info.a_item_id,
            state: ss_drone_info.state.into(),
        }
    }
}
