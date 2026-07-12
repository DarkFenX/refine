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
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<CreatedFleetIdResp, HExecError> {
        let mut core_fleet = core_sol.create_fleet();
        for fit_id in &self.fit_ids {
            core_fleet.add_fit(fit_id).map_err(|error| match error {
                rc::err::FleetAddFitError::FitNotFound(e) => HExecError::FitNotFoundSecondary(e),
                rc::err::FleetAddFitError::FitAlreadyInThisFleet(e) => HExecError::FitAlreadyInThisFleet(e),
            })?;
        }
        Ok(CreatedFleetIdResp::from_core_fleet(core_fleet))
    }
}
