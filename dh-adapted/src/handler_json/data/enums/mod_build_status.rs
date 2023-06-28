#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModBuildStatus {
    Unbuilt,
    Error(rc::Amount),
    SuccessPartial(rc::Amount),
    Success,
    Custom,
}
impl From<&rc::consts::ModBuildStatus> for CModBuildStatus {
    fn from(mod_build_status: &rc::consts::ModBuildStatus) -> Self {
        match mod_build_status {
            rc::consts::ModBuildStatus::Unbuilt => Self::Unbuilt,
            rc::consts::ModBuildStatus::Error(fails) => Self::Error(*fails),
            rc::consts::ModBuildStatus::SuccessPartial(fails) => Self::SuccessPartial(*fails),
            rc::consts::ModBuildStatus::Success => Self::Success,
            rc::consts::ModBuildStatus::Custom => Self::Custom,
        }
    }
}
impl Into<rc::consts::ModBuildStatus> for &CModBuildStatus {
    fn into(self) -> rc::consts::ModBuildStatus {
        match self {
            CModBuildStatus::Unbuilt => rc::consts::ModBuildStatus::Unbuilt,
            CModBuildStatus::Error(fails) => rc::consts::ModBuildStatus::Error(*fails),
            CModBuildStatus::SuccessPartial(fails) => rc::consts::ModBuildStatus::SuccessPartial(*fails),
            CModBuildStatus::Success => rc::consts::ModBuildStatus::Success,
            CModBuildStatus::Custom => rc::consts::ModBuildStatus::Custom,
        }
    }
}
