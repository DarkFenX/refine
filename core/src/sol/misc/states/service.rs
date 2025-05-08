use crate::ad;

/// Service states.
#[derive(Copy, Clone)]
pub enum ServiceState {
    /// Service will receive modifications, but will not apply its modifications to anything else.
    Ghost,
    Offline,
    Online,
}
impl From<ad::AState> for ServiceState {
    fn from(a_state: ad::AState) -> Self {
        match a_state {
            ad::AState::Ghost => Self::Ghost,
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
            ServiceState::Ghost => Self::Ghost,
            ServiceState::Offline => Self::Offline,
            ServiceState::Online => Self::Online,
        }
    }
}
