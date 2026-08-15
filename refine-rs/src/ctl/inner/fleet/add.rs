use crate::{AddedFleetIdResp, CtlCmdResps, FitId, FitIdBackref, err::BackrefRenderError};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdFleetAddFCtxBIds {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) fit_ids: Vec<FitIdBackref> = Vec::new(),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdFleetAddFCtxRIds {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) fit_ids: Vec<FitId> = Vec::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFleetAddFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdFleetAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdFleetAddFCtxRIds {
            fit_ids: resps.render_fit_ids(self.fit_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFleetAddFCtxRIds {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<AddedFleetIdResp, AddFleetError> {
        let mut core_fleet = core_sol.add_fleet();
        for fit_id in &self.fit_ids {
            core_fleet.add_fit(fit_id)?;
        }
        Ok(AddedFleetIdResp::from_core_fleet(core_fleet))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddFleetError {
    #[error("failed to add fit to fleet")]
    FitAdd(#[from] rc::err::FleetAddFitError),
}
