use crate::info::HFleetInfoMode;

#[derive(serde::Deserialize)]
pub(crate) struct HFleetInfoParams {
    pub(crate) fleet: Option<HFleetInfoMode>,
}
