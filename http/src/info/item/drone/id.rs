#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HDroneInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&rc::DroneInfo> for HDroneInfoId {
    fn from(core_drone_info: &rc::DroneInfo) -> Self {
        Self { id: core_drone_info.id }
    }
}
