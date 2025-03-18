use crate::sol::uad::item::SolItemState;

/// Service states.
#[derive(Copy, Clone)]
pub enum SolServiceState {
    /// Service will receive modifications, but will not apply its modifications to anything else.
    Ghost,
    Offline,
    Online,
}
impl From<SolItemState> for SolServiceState {
    fn from(state: SolItemState) -> Self {
        match state {
            SolItemState::Ghost => Self::Ghost,
            SolItemState::Offline => Self::Offline,
            SolItemState::Online => Self::Online,
            SolItemState::Active => Self::Online,
            SolItemState::Overload => Self::Online,
        }
    }
}
impl From<SolServiceState> for SolItemState {
    fn from(state: SolServiceState) -> Self {
        match state {
            SolServiceState::Ghost => Self::Ghost,
            SolServiceState::Offline => Self::Offline,
            SolServiceState::Online => Self::Online,
        }
    }
}
