use crate::rd::RState;

/// Service states.
#[derive(Copy, Clone)]
pub enum ServiceState {
    Disabled,
    Offline,
    Online,
}
impl From<RState> for ServiceState {
    fn from(r_state: RState) -> Self {
        match r_state {
            RState::Ghost => Self::Disabled,
            RState::Disabled => Self::Disabled,
            RState::Offline => Self::Offline,
            RState::Online => Self::Online,
            RState::Active => Self::Online,
            RState::Overload => Self::Online,
        }
    }
}
impl From<ServiceState> for RState {
    fn from(service_state: ServiceState) -> Self {
        match service_state {
            ServiceState::Disabled => Self::Disabled,
            ServiceState::Offline => Self::Offline,
            ServiceState::Online => Self::Online,
        }
    }
}
