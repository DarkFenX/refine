#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModBuildStatus {
    Unbuilt,
    Error(rc::Amount),
    SuccessPartial(rc::Amount),
    Success,
    Custom,
}
impl From<&rc::ad::AEffectModBuildStatus> for CModBuildStatus {
    fn from(mod_build_status: &rc::ad::AEffectModBuildStatus) -> Self {
        match mod_build_status {
            rc::ad::AEffectModBuildStatus::Unbuilt => Self::Unbuilt,
            rc::ad::AEffectModBuildStatus::Error(fails) => Self::Error(*fails),
            rc::ad::AEffectModBuildStatus::SuccessPartial(fails) => Self::SuccessPartial(*fails),
            rc::ad::AEffectModBuildStatus::Success => Self::Success,
            rc::ad::AEffectModBuildStatus::Custom => Self::Custom,
        }
    }
}
impl Into<rc::ad::AEffectModBuildStatus> for &CModBuildStatus {
    fn into(self) -> rc::ad::AEffectModBuildStatus {
        match self {
            CModBuildStatus::Unbuilt => rc::ad::AEffectModBuildStatus::Unbuilt,
            CModBuildStatus::Error(fails) => rc::ad::AEffectModBuildStatus::Error(*fails),
            CModBuildStatus::SuccessPartial(fails) => rc::ad::AEffectModBuildStatus::SuccessPartial(*fails),
            CModBuildStatus::Success => rc::ad::AEffectModBuildStatus::Success,
            CModBuildStatus::Custom => rc::ad::AEffectModBuildStatus::Custom,
        }
    }
}
