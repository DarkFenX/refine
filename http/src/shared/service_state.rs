#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HServiceState {
    Ghost,
    Offline,
    Online,
}
impl From<&rc::SolServiceState> for HServiceState {
    fn from(core_service_state: &rc::SolServiceState) -> Self {
        match core_service_state {
            rc::SolServiceState::Ghost => Self::Ghost,
            rc::SolServiceState::Offline => Self::Offline,
            rc::SolServiceState::Online => Self::Online,
        }
    }
}
impl From<&HServiceState> for rc::SolServiceState {
    fn from(h_service_state: &HServiceState) -> Self {
        match h_service_state {
            HServiceState::Ghost => Self::Ghost,
            HServiceState::Offline => Self::Offline,
            HServiceState::Online => Self::Online,
        }
    }
}
