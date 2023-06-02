#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json::cdt) enum ModBuildStatus {
    Unbuilt,
    Error(rc::ReeInt),
    SuccessPartial(rc::ReeInt),
    Success,
    Custom,
}
impl From<rc::consts::ModBuildStatus> for ModBuildStatus {
    fn from(value: rc::consts::ModBuildStatus) -> Self {
        match value {
            rc::consts::ModBuildStatus::Unbuilt => Self::Unbuilt,
            rc::consts::ModBuildStatus::Error(fails) => Self::Error(fails),
            rc::consts::ModBuildStatus::SuccessPartial(fails) => Self::SuccessPartial(fails),
            rc::consts::ModBuildStatus::Success => Self::Success,
            rc::consts::ModBuildStatus::Custom => Self::Custom,
        }
    }
}
impl Into<rc::consts::ModBuildStatus> for ModBuildStatus {
    fn into(self) -> rc::consts::ModBuildStatus {
        match self {
            ModBuildStatus::Unbuilt => rc::consts::ModBuildStatus::Unbuilt,
            ModBuildStatus::Error(fails) => rc::consts::ModBuildStatus::Error(fails),
            ModBuildStatus::SuccessPartial(fails) => rc::consts::ModBuildStatus::SuccessPartial(fails),
            ModBuildStatus::Success => rc::consts::ModBuildStatus::Success,
            ModBuildStatus::Custom => rc::consts::ModBuildStatus::Custom,
        }
    }
}
