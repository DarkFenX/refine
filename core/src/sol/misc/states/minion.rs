use crate::ad;

/// Drone/fighter states.
#[derive(Copy, Clone)]
pub enum MinionState {
    InBay,
    InSpace,
    Engaging,
}
impl From<ad::AState> for MinionState {
    fn from(a_state: ad::AState) -> Self {
        match a_state {
            ad::AState::Ghost => Self::InBay,
            ad::AState::Offline => Self::InBay,
            ad::AState::Online => Self::InSpace,
            ad::AState::Active => Self::Engaging,
            ad::AState::Overload => Self::Engaging,
        }
    }
}
impl From<MinionState> for ad::AState {
    fn from(minion_state: MinionState) -> Self {
        match minion_state {
            MinionState::InBay => Self::Ghost,
            MinionState::InSpace => Self::Online,
            MinionState::Engaging => Self::Active,
        }
    }
}
