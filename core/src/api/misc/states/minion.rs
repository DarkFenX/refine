use crate::rd::RState;

/// Drone/fighter states.
#[derive(Copy, Clone)]
pub enum MinionState {
    InBay,
    InSpace,
    Engaging,
}
impl From<RState> for MinionState {
    fn from(r_state: RState) -> Self {
        match r_state {
            RState::Ghost => Self::InBay,
            RState::Disabled => Self::InBay,
            RState::Offline => Self::InBay,
            RState::Online => Self::InSpace,
            RState::Active => Self::Engaging,
            RState::Overload => Self::Engaging,
        }
    }
}
impl From<MinionState> for RState {
    fn from(minion_state: MinionState) -> Self {
        match minion_state {
            MinionState::InBay => Self::Offline,
            MinionState::InSpace => Self::Online,
            MinionState::Engaging => Self::Active,
        }
    }
}
