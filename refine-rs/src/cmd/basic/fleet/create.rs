use crate::cmd::CreatedFleetIdResp;

// Commands with full context
#[derive(Default)]
pub(in crate::cmd) struct FleetCreateCmdFCtxRIds {
    pub(in crate::cmd) fit_ids: Vec<rc::FitId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetCreateCmdFCtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<CreatedFleetIdResp, CreateFleetError> {
        let mut core_fleet = core_sol.create_fleet();
        for fit_id in &self.fit_ids {
            core_fleet.add_fit(fit_id)?;
        }
        Ok(CreatedFleetIdResp::from_core_fleet(core_fleet))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateFleetError {
    #[error("failed to create fleet: {0}")]
    FitAddFailed(#[from] rc::err::FleetAddFitError),
}
