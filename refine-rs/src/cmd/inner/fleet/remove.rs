use crate::cmd::{BackrefRenderError, CmdResps, FleetIdBackref};

// Commands with full context
pub(in crate::cmd) struct CmdFleetRemoveFCtxBIds {
    pub(in crate::cmd) fleet_id: FleetIdBackref,
    pub(in crate::cmd) ictx_cmd: CmdFleetRemoveICtx,
}
pub(crate) struct CmdFleetRemoveFCtxRIds {
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
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), GetFleetRemoveFleetError> {
        let core_fleet = core_sol.get_fleet_mut(&self.fleet_id)?;
        Ok(self.ictx_cmd.execute(core_fleet))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFleetRemoveFleetError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFleetError),
}

impl CmdFleetRemoveICtx {
    pub(in crate::cmd) fn execute(&self, core_fleet: rc::FleetMut) {
        core_fleet.remove()
    }
}
