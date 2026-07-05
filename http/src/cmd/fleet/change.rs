use serde::Deserialize;

use crate::{cmd::basic::HFleetChangeCmdICtxRIds, err::HExecError};

#[derive(Default, Deserialize)]
pub(crate) struct HFleetChangeCmd {
    #[serde(flatten)]
    basic: HFleetChangeCmdICtxRIds,
}
impl HFleetChangeCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fleet_id: &rc::FleetId) -> Result<(), HExecError> {
        self.basic.execute(core_sol, fleet_id)
    }
}
