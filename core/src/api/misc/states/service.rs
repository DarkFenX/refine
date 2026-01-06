use crate::rd::RState;

/// Service states.
#[derive(Copy, Clone)]
pub enum ServiceState {
    Disabled,
    Offline,
    Online,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ServiceState {
    pub(crate) fn from_r_state(r_state: RState) -> Self {
        match r_state {
            RState::Ghost => Self::Disabled,
            RState::Disabled => Self::Disabled,
            RState::Offline => Self::Offline,
            RState::Online => Self::Online,
            RState::Active => Self::Online,
            RState::Overload => Self::Online,
        }
    }
    pub(crate) fn into_r_state(self) -> RState {
        match self {
            ServiceState::Disabled => RState::Disabled,
            ServiceState::Offline => RState::Offline,
            ServiceState::Online => RState::Online,
        }
    }
}
