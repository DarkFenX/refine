use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HModuleState {
    Disabled,
    Offline,
    Online,
    Active,
    Overload,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HModuleState {
    pub(crate) fn from_core(core_module_state: rc::ModuleState) -> Self {
        match core_module_state {
            rc::ModuleState::Disabled => Self::Disabled,
            rc::ModuleState::Offline => Self::Offline,
            rc::ModuleState::Online => Self::Online,
            rc::ModuleState::Active => Self::Active,
            rc::ModuleState::Overload => Self::Overload,
        }
    }
    pub(crate) fn into_core(self) -> rc::ModuleState {
        match self {
            Self::Disabled => rc::ModuleState::Disabled,
            Self::Offline => rc::ModuleState::Offline,
            Self::Online => rc::ModuleState::Online,
            Self::Active => rc::ModuleState::Active,
            Self::Overload => rc::ModuleState::Overload,
        }
    }
}
