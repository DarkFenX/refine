use crate::cmd::{AddedFitIdResp, BackrefRenderError, CmdResps, FleetIdBackref};

// Commands with full context
pub(in crate::cmd) struct ICmdFitAddFCtxBIds {
    pub(in crate::cmd) shared: ICmdFitAddShared = ICmdFitAddShared { .. },
    pub(in crate::cmd) fleet_id: Option<FleetIdBackref> = None,
}
pub(crate) struct ICmdFitAddFCtxRIds {
    pub(in crate::cmd) shared: ICmdFitAddShared = ICmdFitAddShared { .. },
    pub(in crate::cmd) fleet_id: Option<rc::FleetId> = None,
}
pub(in crate::cmd) struct ICmdFitAddShared {
    pub(in crate::cmd) sec_status: Option<rc::FitSecStatus> = None,
    pub(in crate::cmd) rah_incoming_dps: Option<rc::DpsProfile> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFitAddFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFitAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdFitAddFCtxRIds {
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
impl ICmdFitAddFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<AddedFitIdResp, AddFitError> {
        let mut core_fit = core_sol.add_fit();
        if let Some(fleet_id) = self.fleet_id {
            core_fit.set_fleet(&fleet_id)?;
        }
        if let Some(sec_status) = self.shared.sec_status {
            core_fit.set_sec_status(sec_status);
        }
        if let Some(rah_incoming_dps) = self.shared.rah_incoming_dps {
            core_fit.set_rah_incoming_dps(rah_incoming_dps);
        }
        Ok(AddedFitIdResp::from_core_fit(core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddFitError {
    #[error("failed to set fleet: {0}")]
    FleetSetFailed(#[from] rc::err::SetFitFleetError),
}
