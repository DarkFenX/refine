use crate::{CmdResps, FleetId, FleetIdBackref, err::BackrefRenderError};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdFleetRemoveFCtxBIds {
    pub(in crate::cmd) fleet_id: FleetIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) ictx_cmd: ICmdFleetRemoveICtx = ICmdFleetRemoveICtx,
}
pub(crate) struct ICmdFleetRemoveFCtxRIds {
    fleet_id: FleetId,
    ictx_cmd: ICmdFleetRemoveICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdFleetRemoveICtx;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFleetRemoveFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFleetRemoveFCtxRIds, BackrefRenderError> {
        Ok(ICmdFleetRemoveFCtxRIds {
            fleet_id: resps.render_fleet_id(self.fleet_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFleetRemoveFCtxRIds {
    pub(in crate::cmd) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), GetFleetRemoveFleetError> {
        let core_fleet = core_sol.get_fleet_mut(&self.fleet_id)?;
        Ok(self.ictx_cmd.execute(core_fleet))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFleetRemoveFleetError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFleetError),
}

impl ICmdFleetRemoveICtx {
    pub(in crate::cmd) fn execute(self, core_fleet: rc::FleetMut) {
        core_fleet.remove()
    }
}
