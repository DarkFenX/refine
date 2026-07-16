use crate::{CmdResps, FitIdBackref, FleetIdBackref, err::BackrefRenderError};

// Commands with full context
pub(in crate::cmd) struct ICmdFleetChangeFCtxBIds {
    pub(in crate::cmd) fleet_id: FleetIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdFleetChangeICtxBIds = ICmdFleetChangeICtxBIds { .. },
}
pub(crate) struct ICmdFleetChangeFCtxRIds {
    fleet_id: rc::FleetId,
    ictx_cmd: ICmdFleetChangeICtxRIds,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdFleetChangeICtxBIds {
    pub(in crate::cmd) add_fit_ids: Vec<FitIdBackref> = Vec::new(),
    pub(in crate::cmd) rm_fit_ids: Vec<FitIdBackref> = Vec::new(),
}
pub(in crate::cmd) struct ICmdFleetChangeICtxRIds {
    pub(in crate::cmd) add_fit_ids: Vec<rc::FitId> = Vec::new(),
    pub(in crate::cmd) rm_fit_ids: Vec<rc::FitId> = Vec::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFleetChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFleetChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdFleetChangeFCtxRIds {
            fleet_id: resps.render_fleet_id(self.fleet_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdFleetChangeICtxBIds {
    fn render(self, resps: &CmdResps) -> Result<ICmdFleetChangeICtxRIds, BackrefRenderError> {
        Ok(ICmdFleetChangeICtxRIds {
            add_fit_ids: resps.render_fit_ids(self.add_fit_ids)?,
            rm_fit_ids: resps.render_fit_ids(self.rm_fit_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFleetChangeFCtxRIds {
    pub(in crate::cmd) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), GetFleetChangeFleetError> {
        let mut core_fleet = core_sol.get_fleet_mut(&self.fleet_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fleet)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFleetChangeFleetError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFleetError),
    #[error("{0}")]
    ChangeFailed(#[from] FleetChangeFleetError),
}

impl ICmdFleetChangeICtxRIds {
    pub(in crate::cmd) fn execute(self, core_fleet: &mut rc::FleetMut) -> Result<(), FleetChangeFleetError> {
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
pub enum FleetChangeFleetError {
    #[error("failed to add fit: {0}")]
    FitAddFailed(#[from] rc::err::FleetAddFitError),
    #[error("failed to remove fit: {0}")]
    FitRemoveFailed(#[from] rc::err::FleetRemoveFitError),
}
