use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HServiceState {
    Disabled,
    Offline,
    Online,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HServiceState {
    pub(crate) fn from_core(core_service_state: rc::ServiceState) -> Self {
        match core_service_state {
            rc::ServiceState::Disabled => Self::Disabled,
            rc::ServiceState::Offline => Self::Offline,
            rc::ServiceState::Online => Self::Online,
        }
    }
    pub(crate) fn into_core(self) -> rc::ServiceState {
        match self {
            Self::Disabled => rc::ServiceState::Disabled,
            Self::Offline => rc::ServiceState::Offline,
            Self::Online => rc::ServiceState::Online,
        }
    }
}
