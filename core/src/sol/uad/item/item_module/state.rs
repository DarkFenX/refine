use crate::ad;

/// Module states.
#[derive(Copy, Clone)]
pub enum ModuleState {
    /// Module will receive modifications, but will not apply its modifications to anything else.
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl From<ad::AState> for ModuleState {
    fn from(a_state: ad::AState) -> Self {
        match a_state {
            ad::AState::Ghost => Self::Ghost,
            ad::AState::Offline => Self::Offline,
            ad::AState::Online => Self::Online,
            ad::AState::Active => Self::Active,
            ad::AState::Overload => Self::Overload,
        }
    }
}
impl From<ModuleState> for ad::AState {
    fn from(module_state: ModuleState) -> Self {
        match module_state {
            ModuleState::Ghost => Self::Ghost,
            ModuleState::Offline => Self::Offline,
            ModuleState::Online => Self::Online,
            ModuleState::Active => Self::Active,
            ModuleState::Overload => Self::Overload,
        }
    }
}
