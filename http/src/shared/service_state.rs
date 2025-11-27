#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HServiceState {
    Disabled,
    Offline,
    Online,
}
impl From<&rc::ServiceState> for HServiceState {
    fn from(core_service_state: &rc::ServiceState) -> Self {
        match core_service_state {
            rc::ServiceState::Disabled => Self::Disabled,
            rc::ServiceState::Offline => Self::Offline,
            rc::ServiceState::Online => Self::Online,
        }
    }
}
impl From<&HServiceState> for rc::ServiceState {
    fn from(h_service_state: &HServiceState) -> Self {
        match h_service_state {
            HServiceState::Disabled => Self::Disabled,
            HServiceState::Offline => Self::Offline,
            HServiceState::Online => Self::Online,
        }
    }
}
