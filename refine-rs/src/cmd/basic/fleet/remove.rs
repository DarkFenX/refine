use crate::cmd::{BackrefRenderError, CmdResps, FleetIdBackref};

// Commands with full context
struct CmdFleetRemoveFCtxBIds {
    fleet_id: FleetIdBackref,
    ictx_cmd: CmdFleetRemoveICtx,
}
struct CmdFleetRemoveFCtxRIds {
    fleet_id: rc::FleetId,
    ictx_cmd: CmdFleetRemoveICtx,
}

// Commands with incomplete context
#[derive(Default)]
pub(in crate::cmd) struct CmdFleetRemoveICtx;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdFleetRemoveFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<CmdFleetRemoveFCtxRIds, BackrefRenderError> {
        Ok(CmdFleetRemoveFCtxRIds {
            fleet_id: resps.render_fleet_id(self.fleet_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdFleetRemoveFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), RemoveFleetError> {
        self.ictx_cmd.execute(core_sol, &self.fleet_id)
    }
}

impl CmdFleetRemoveICtx {
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
    #[error("{0}")]
    FleetGetFailed(#[from] rc::err::GetFleetError),
}
