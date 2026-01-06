use crate::rd::RState;

/// Drone/fighter states.
#[derive(Copy, Clone)]
pub enum MinionState {
    InBay,
    InSpace,
    Engaging,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl MinionState {
    pub(crate) fn from_r_state(r_state: RState) -> Self {
        match r_state {
            RState::Ghost => Self::InBay,
            RState::Disabled => Self::InBay,
            RState::Offline => Self::InBay,
            RState::Online => Self::InSpace,
            RState::Active => Self::Engaging,
            RState::Overload => Self::Engaging,
        }
    }
    pub(crate) fn into_r_state(self) -> RState {
        match self {
            MinionState::InBay => RState::Offline,
            MinionState::InSpace => RState::Online,
            MinionState::Engaging => RState::Active,
        }
    }
}
