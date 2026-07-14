use crate::cmd::{BackrefRenderError, CmdResps, CreatedFitIdResp, FleetIdBackref};

// Commands with full context
#[derive(Default)]
pub(in crate::cmd) struct ICmdFitCreateFCtxBIds {
    pub(in crate::cmd) shared: ICmdFitCreateShared,
    pub(in crate::cmd) fleet_id: Option<FleetIdBackref>,
}
#[derive(Default)]
pub(crate) struct ICmdFitCreateFCtxRIds {
    pub(in crate::cmd) shared: ICmdFitCreateShared,
    pub(in crate::cmd) fleet_id: Option<rc::FleetId>,
}
#[derive(Default)]
pub(in crate::cmd) struct ICmdFitCreateShared {
    pub(in crate::cmd) sec_status: Option<rc::FitSecStatus>,
    pub(in crate::cmd) rah_incoming_dps: Option<rc::DpsProfile>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFitCreateFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFitCreateFCtxRIds, BackrefRenderError> {
        Ok(ICmdFitCreateFCtxRIds {
            shared: self.shared,
            fleet_id: match self.fleet_id {
                Some(fleet_id) => Some(resps.render_fleet_id(fleet_id)?),
                None => None,
            },
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFitCreateFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<CreatedFitIdResp, CreateFitError> {
        let mut core_fit = core_sol.create_fit();
        if let Some(fleet_id) = self.fleet_id {
            core_fit.set_fleet(&fleet_id)?;
        }
        if let Some(sec_status) = self.shared.sec_status {
            core_fit.set_sec_status(sec_status);
        }
        if let Some(rah_incoming_dps) = self.shared.rah_incoming_dps {
            core_fit.set_rah_incoming_dps(rah_incoming_dps);
        }
        Ok(CreatedFitIdResp::from_core_fit(core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateFitError {
    #[error("failed to set fleet: {0}")]
    FleetSetFailed(#[from] rc::err::SetFitFleetError),
}
