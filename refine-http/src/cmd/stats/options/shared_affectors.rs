use serde::Deserialize;

#[derive(Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HStatAffectors {
    #[default]
    Unmodified,
    Deactivate,
    Offline,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatAffectors {
    pub(in crate::cmd::stats) fn into_core(self) -> rc::CtlAffectors {
        match self {
            Self::Unmodified => rc::CtlAffectors::Unmodified,
            Self::Deactivate => rc::CtlAffectors::Deactivate,
            Self::Offline => rc::CtlAffectors::Offline,
        }
    }
}
