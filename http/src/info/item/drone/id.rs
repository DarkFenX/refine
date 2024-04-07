#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HDroneInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsDroneInfo> for HDroneInfoId {
    fn from(core_drone_info: &rc::SsDroneInfo) -> Self {
        Self { id: core_drone_info.id }
    }
}
