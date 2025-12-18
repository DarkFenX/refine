use crate::ad;

/// Service states.
#[derive(Copy, Clone)]
pub enum ServiceState {
    Disabled,
    Offline,
    Online,
}
impl From<ad::AState> for ServiceState {
    fn from(a_state: ad::AState) -> Self {
        match a_state {
            ad::AState::Ghost => Self::Disabled,
            ad::AState::Disabled => Self::Disabled,
            ad::AState::Offline => Self::Offline,
            ad::AState::Online => Self::Online,
            ad::AState::Active => Self::Online,
            ad::AState::Overload => Self::Online,
        }
    }
}
impl From<ServiceState> for ad::AState {
    fn from(service_state: ServiceState) -> Self {
        match service_state {
            ServiceState::Disabled => Self::Disabled,
            ServiceState::Offline => Self::Offline,
            ServiceState::Online => Self::Online,
        }
    }
}
