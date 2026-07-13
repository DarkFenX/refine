use crate::cmd::{BackrefRenderError, CmdResps, FleetIdBackref};

// Commands with full context
struct FleetRemoveCmdFCtxBIds {
    fleet_id: FleetIdBackref,
    ictx_cmd: FleetRemoveCmdICtx,
}
struct FleetRemoveCmdFCtxRIds {
    fleet_id: rc::FleetId,
    ictx_cmd: FleetRemoveCmdICtx,
}

// Commands with incomplete context
#[derive(Default)]
pub(in crate::cmd) struct FleetRemoveCmdICtx;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetRemoveCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<FleetRemoveCmdFCtxRIds, BackrefRenderError> {
        Ok(FleetRemoveCmdFCtxRIds {
            fleet_id: resps.render_fleet_id(self.fleet_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetRemoveCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), RemoveFleetError> {
        self.ictx_cmd.execute(core_sol, &self.fleet_id)
    }
}

impl FleetRemoveCmdICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fleet_id: &rc::FleetId,
    ) -> Result<(), RemoveFleetError> {
        core_sol.get_fleet_mut(fleet_id)?.remove();
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveFleetError {
    #[error("failed to remove fleet: {0}")]
    FleetGetFailed(#[from] rc::err::GetFleetError),
}
