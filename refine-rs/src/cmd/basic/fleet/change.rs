use crate::cmd::{BackrefRenderError, CmdResps, FitIdBackref, FleetIdBackref};

// Commands with full context
struct CmdFleetChangeFCtxBIds {
    fleet_id: FleetIdBackref,
    ictx_cmd: CmdFleetChangeICtxBIds,
}
struct CmdFleetChangeFCtxRIds {
    fleet_id: rc::FleetId,
    ictx_cmd: CmdFleetChangeICtxRIds,
}

// Commands with incomplete context
struct CmdFleetChangeICtxBIds {
    add_fit_ids: Vec<FitIdBackref>,
    rm_fit_ids: Vec<FitIdBackref>,
}
#[derive(Default)]
pub(in crate::cmd) struct CmdFleetChangeICtxRIds {
    pub(in crate::cmd) add_fit_ids: Vec<rc::FitId>,
    pub(in crate::cmd) rm_fit_ids: Vec<rc::FitId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdFleetChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<CmdFleetChangeFCtxRIds, BackrefRenderError> {
        Ok(CmdFleetChangeFCtxRIds {
            fleet_id: resps.render_fleet_id(self.fleet_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl CmdFleetChangeICtxBIds {
    fn render(self, resps: &CmdResps) -> Result<CmdFleetChangeICtxRIds, BackrefRenderError> {
        Ok(CmdFleetChangeICtxRIds {
            add_fit_ids: resps.render_fit_ids(self.add_fit_ids)?,
            rm_fit_ids: resps.render_fit_ids(self.rm_fit_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdFleetChangeFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), BasicChangeFleetError> {
        self.ictx_cmd.execute(core_sol, &self.fleet_id)
    }
}

impl CmdFleetChangeICtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fleet_id: &rc::FleetId,
    ) -> Result<(), BasicChangeFleetError> {
        let mut core_fleet = core_sol.get_fleet_mut(fleet_id)?;
        for fit_id in self.rm_fit_ids.iter() {
            core_fleet.remove_fit(fit_id)?;
        }
        for fit_id in self.add_fit_ids.iter() {
            core_fleet.add_fit(fit_id)?;
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BasicChangeFleetError {
    #[error("{0}")]
    FleetGetFailed(#[from] rc::err::GetFleetError),
    #[error("failed to add fit: {0}")]
    FitAddFailed(#[from] rc::err::FleetAddFitError),
    #[error("failed to remove fit: {0}")]
    FitRemoveFailed(#[from] rc::err::FleetRemoveFitError),
}
