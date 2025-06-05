use crate::info::HFleetInfoMode;

#[derive(serde::Deserialize)]
pub(crate) struct HFleetInfoParams {
    pub(in crate::handlers::fleet) fleet: Option<HFleetInfoMode>,
}
