use serde::Deserialize;

use crate::{cmd::basic::HFleetRemoveCmdICtx, util::HExecError};

#[derive(Default, Deserialize)]
pub(crate) struct HFleetRemoveCmd {
    #[serde(flatten)]
    basic: HFleetRemoveCmdICtx,
}
impl HFleetRemoveCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fleet_id: &rc::FleetId) -> Result<(), HExecError> {
        self.basic.execute(core_sol, fleet_id)
    }
}
