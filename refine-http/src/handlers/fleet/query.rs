use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct FleetInfoParams {
    pub(super) fleet: Option<rs::FleetInfoMode>,
}
