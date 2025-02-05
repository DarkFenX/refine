use crate::sol::uad::item::SolItemState;

/// Module states.
#[derive(Copy, Clone)]
pub enum SolModuleState {
    /// Module will receive modifications, but will not apply its modifications to anything else.
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl From<SolItemState> for SolModuleState {
    fn from(state: SolItemState) -> Self {
        match state {
            SolItemState::Ghost => Self::Ghost,
            SolItemState::Offline => Self::Offline,
            SolItemState::Online => Self::Online,
            SolItemState::Active => Self::Active,
            SolItemState::Overload => Self::Overload,
        }
    }
}
impl From<SolModuleState> for SolItemState {
    fn from(state: SolModuleState) -> Self {
        match state {
            SolModuleState::Ghost => Self::Ghost,
            SolModuleState::Offline => Self::Offline,
            SolModuleState::Online => Self::Online,
            SolModuleState::Active => Self::Active,
            SolModuleState::Overload => Self::Overload,
        }
    }
}
