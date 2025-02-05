#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HModuleState {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl From<&rc::SolModuleState> for HModuleState {
    fn from(core_module_state: &rc::SolModuleState) -> Self {
        match core_module_state {
            rc::SolModuleState::Offline => Self::Offline,
            rc::SolModuleState::Online => Self::Online,
            rc::SolModuleState::Active => Self::Active,
            rc::SolModuleState::Ghost => Self::Ghost,
            rc::SolModuleState::Overload => Self::Overload,
        }
    }
}
impl From<&HModuleState> for rc::SolModuleState {
    fn from(h_module_state: &HModuleState) -> Self {
        match h_module_state {
            HModuleState::Offline => Self::Offline,
            HModuleState::Online => Self::Online,
            HModuleState::Active => Self::Active,
            HModuleState::Ghost => Self::Ghost,
            HModuleState::Overload => Self::Overload,
        }
    }
}
