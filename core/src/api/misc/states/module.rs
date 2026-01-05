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
impl From<RState> for ModuleState {
    fn from(r_state: RState) -> Self {
        match r_state {
            RState::Ghost => Self::Disabled,
            RState::Disabled => Self::Disabled,
            RState::Offline => Self::Offline,
            RState::Online => Self::Online,
            RState::Active => Self::Active,
            RState::Overload => Self::Overload,
        }
    }
}
impl From<ModuleState> for RState {
    fn from(module_state: ModuleState) -> Self {
        match module_state {
            ModuleState::Disabled => Self::Disabled,
            ModuleState::Offline => Self::Offline,
            ModuleState::Online => Self::Online,
            ModuleState::Active => Self::Active,
            ModuleState::Overload => Self::Overload,
        }
    }
}
