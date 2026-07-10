use crate::rd::RState;

/// Module states.
#[derive(Copy, Clone)]
pub enum ModuleState {
    Disabled,
    Offline,
    Online,
    Active,
    Overload,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ModuleState {
    pub(crate) fn from_r_state(r_state: RState) -> Self {
        match r_state {
            RState::Ghost => Self::Disabled,
            RState::Disabled => Self::Disabled,
            RState::Offline => Self::Offline,
            RState::Online => Self::Online,
            RState::Active => Self::Active,
            RState::Overload => Self::Overload,
        }
    }
    pub(crate) fn into_r_state(self) -> RState {
        match self {
            ModuleState::Disabled => RState::Disabled,
            ModuleState::Offline => RState::Offline,
            ModuleState::Online => RState::Online,
            ModuleState::Active => RState::Active,
            ModuleState::Overload => RState::Overload,
        }
    }
}
