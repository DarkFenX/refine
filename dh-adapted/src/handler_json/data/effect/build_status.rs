use crate::handler_json::data::CCount;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModBuildStatus {
    Unbuilt,
    Error(CCount),
    SuccessPartial(CCount),
    Success,
    Custom,
}
impl From<&rc::ad::AEffectModBuildStatus> for CModBuildStatus {
    fn from(a_mod_build_status: &rc::ad::AEffectModBuildStatus) -> Self {
        match a_mod_build_status {
            rc::ad::AEffectModBuildStatus::Unbuilt => Self::Unbuilt,
            rc::ad::AEffectModBuildStatus::Error(fails) => Self::Error(*fails),
            rc::ad::AEffectModBuildStatus::SuccessPartial(fails) => Self::SuccessPartial(*fails),
            rc::ad::AEffectModBuildStatus::Success => Self::Success,
            rc::ad::AEffectModBuildStatus::Custom => Self::Custom,
        }
    }
}
impl From<&CModBuildStatus> for rc::ad::AEffectModBuildStatus {
    fn from(c_mod_build_status: &CModBuildStatus) -> Self {
        match c_mod_build_status {
            CModBuildStatus::Unbuilt => Self::Unbuilt,
            CModBuildStatus::Error(fails) => Self::Error(*fails),
            CModBuildStatus::SuccessPartial(fails) => Self::SuccessPartial(*fails),
            CModBuildStatus::Success => Self::Success,
            CModBuildStatus::Custom => Self::Custom,
        }
    }
}
