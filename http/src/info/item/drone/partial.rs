use crate::shared::HState;

#[derive(serde::Serialize)]
pub(crate) struct HDroneInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: HState,
}
impl From<&rc::SsDroneInfo> for HDroneInfoPartial {
    fn from(core_drone_info: &rc::SsDroneInfo) -> Self {
        Self {
            id: core_drone_info.id,
            fit_id: core_drone_info.fit_id,
            type_id: core_drone_info.a_item_id,
            state: (&core_drone_info.state).into(),
        }
    }
}
