use crate::cmd::{BackrefRenderError, CmdResps, CreatedFleetIdResp, FitIdBackref};

// Commands with full context
#[derive(Default)]
pub(in crate::cmd) struct ICmdFleetCreateFCtxBIds {
    pub(in crate::cmd) fit_ids: Vec<FitIdBackref>,
}

#[derive(Default)]
pub(crate) struct ICmdFleetCreateFCtxRIds {
    pub(in crate::cmd) fit_ids: Vec<rc::FitId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFleetCreateFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFleetCreateFCtxRIds, BackrefRenderError> {
        Ok(ICmdFleetCreateFCtxRIds {
            fit_ids: resps.render_fit_ids(self.fit_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFleetCreateFCtxRIds {
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
    #[error("failed to add fit to fleet: {0}")]
    FitAddFailed(#[from] rc::err::FleetAddFitError),
}
