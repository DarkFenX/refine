#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModBuildStatus {
    Unbuilt,
    Error(rc::Amount),
    SuccessPartial(rc::Amount),
    Success,
    Custom,
}
impl From<&rc::ad::AModBuildStatus> for CModBuildStatus {
    fn from(mod_build_status: &rc::ad::AModBuildStatus) -> Self {
        match mod_build_status {
            rc::ad::AModBuildStatus::Unbuilt => Self::Unbuilt,
            rc::ad::AModBuildStatus::Error(fails) => Self::Error(*fails),
            rc::ad::AModBuildStatus::SuccessPartial(fails) => Self::SuccessPartial(*fails),
            rc::ad::AModBuildStatus::Success => Self::Success,
            rc::ad::AModBuildStatus::Custom => Self::Custom,
        }
    }
}
impl Into<rc::ad::AModBuildStatus> for &CModBuildStatus {
    fn into(self) -> rc::ad::AModBuildStatus {
        match self {
            CModBuildStatus::Unbuilt => rc::ad::AModBuildStatus::Unbuilt,
            CModBuildStatus::Error(fails) => rc::ad::AModBuildStatus::Error(*fails),
            CModBuildStatus::SuccessPartial(fails) => rc::ad::AModBuildStatus::SuccessPartial(*fails),
            CModBuildStatus::Success => rc::ad::AModBuildStatus::Success,
            CModBuildStatus::Custom => rc::ad::AModBuildStatus::Custom,
        }
    }
}
