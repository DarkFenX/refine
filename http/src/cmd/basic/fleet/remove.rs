use serde::Deserialize;

use crate::{
    cmd::shared::{HCmdResps, HFleetIdBackref, get_primary_fleet},
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HFleetRemoveCmdFCtxBIds {
    fleet_id: HFleetIdBackref,
    #[serde(flatten)]
    ictx_cmd: HFleetRemoveCmdICtx,
}
pub(crate) struct HFleetRemoveCmdFCtxRIds {
    fleet_id: rc::FleetId,
    ictx_cmd: HFleetRemoveCmdICtx,
}

// Commands with incomplete context
#[derive(Default, Deserialize)]
pub(crate) struct HFleetRemoveCmdICtx;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFleetRemoveCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HFleetRemoveCmdFCtxRIds, HExecError> {
        Ok(HFleetRemoveCmdFCtxRIds {
            fleet_id: resps.render_fleet_id(self.fleet_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFleetRemoveCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fleet_id)
    }
}

impl HFleetRemoveCmdICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fleet_id: &rc::FleetId,
    ) -> Result<(), HExecError> {
        get_primary_fleet(core_sol, fleet_id)?.remove();
        Ok(())
    }
}
