use crate::sol::uad::item::SolItemState;

/// Drone/fighter states.
#[derive(Copy, Clone)]
pub enum SolMinionState {
    InBay,
    InSpace,
    Engaging,
}
impl From<SolItemState> for SolMinionState {
    fn from(state: SolItemState) -> Self {
        match state {
            SolItemState::Ghost => Self::InBay,
            SolItemState::Offline => Self::InBay,
            SolItemState::Online => Self::InSpace,
            SolItemState::Active => Self::Engaging,
            SolItemState::Overload => Self::Engaging,
        }
    }
}
impl From<SolMinionState> for SolItemState {
    fn from(state: SolMinionState) -> Self {
        match state {
            SolMinionState::InBay => Self::Ghost,
            SolMinionState::InSpace => Self::Online,
            SolMinionState::Engaging => Self::Active,
        }
    }
}
