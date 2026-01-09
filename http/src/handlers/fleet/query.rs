use serde::Deserialize;

use crate::info::HFleetInfoMode;

#[derive(Deserialize)]
pub(crate) struct HFleetInfoParams {
    pub(in crate::handlers::fleet) fleet: Option<HFleetInfoMode>,
}
