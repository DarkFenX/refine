use crate::shared::HState;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HDroneInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SsFitId,
    pub(crate) type_id: rc::EItemId,
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
