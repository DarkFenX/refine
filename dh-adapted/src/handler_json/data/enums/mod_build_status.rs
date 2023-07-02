#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModBuildStatus {
    Unbuilt,
    Error(rc::Amount),
    SuccessPartial(rc::Amount),
    Success,
    Custom,
}
impl From<&rc::ad::ModBuildStatus> for CModBuildStatus {
    fn from(mod_build_status: &rc::ad::ModBuildStatus) -> Self {
        match mod_build_status {
            rc::ad::ModBuildStatus::Unbuilt => Self::Unbuilt,
            rc::ad::ModBuildStatus::Error(fails) => Self::Error(*fails),
            rc::ad::ModBuildStatus::SuccessPartial(fails) => Self::SuccessPartial(*fails),
            rc::ad::ModBuildStatus::Success => Self::Success,
            rc::ad::ModBuildStatus::Custom => Self::Custom,
        }
    }
}
impl Into<rc::ad::ModBuildStatus> for &CModBuildStatus {
    fn into(self) -> rc::ad::ModBuildStatus {
        match self {
            CModBuildStatus::Unbuilt => rc::ad::ModBuildStatus::Unbuilt,
            CModBuildStatus::Error(fails) => rc::ad::ModBuildStatus::Error(*fails),
            CModBuildStatus::SuccessPartial(fails) => rc::ad::ModBuildStatus::SuccessPartial(*fails),
            CModBuildStatus::Success => rc::ad::ModBuildStatus::Success,
            CModBuildStatus::Custom => rc::ad::ModBuildStatus::Custom,
        }
    }
}
