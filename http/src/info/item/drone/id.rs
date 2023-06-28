#[derive(serde::Serialize)]
pub(crate) struct HDroneInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsDroneInfo> for HDroneInfoId {
    fn from(core_drone_info: &rc::SsDroneInfo) -> Self {
        Self { id: core_drone_info.id }
    }
}
