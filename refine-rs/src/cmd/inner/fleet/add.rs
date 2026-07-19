use crate::{AddedFleetIdResp, CmdResps, FitId, FitIdBackref, err::BackrefRenderError};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdFleetAddFCtxBIds {
    pub(in crate::cmd) fit_ids: Vec<FitIdBackref> = Vec::new(),
}

pub(crate) struct ICmdFleetAddFCtxRIds {
    pub(in crate::cmd) fit_ids: Vec<FitId> = Vec::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFleetAddFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFleetAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdFleetAddFCtxRIds {
            fit_ids: resps.render_fit_ids(self.fit_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFleetAddFCtxRIds {
    pub(in crate::cmd) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<AddedFleetIdResp, AddFleetError> {
        let mut core_fleet = core_sol.add_fleet();
        for fit_id in &self.fit_ids {
            core_fleet.add_fit(fit_id)?;
        }
        Ok(AddedFleetIdResp::from_core_fleet(core_fleet))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddFleetError {
    #[error("failed to add fit to fleet: {0}")]
    FitAddFailed(#[from] rc::err::FleetAddFitError),
}
