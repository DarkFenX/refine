use crate::{AddedFleetIdResp, CmdResps, FitId, FitIdBr, err::BrResolveError};

// Core commands
pub type FleetAddCmd = FleetAddCmdGen<FitId>;
pub type FleetAddCmdBr = FleetAddCmdGen<FitIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FleetAddCmdGen<F> {
    #[cfg_attr(feature = "serde", serde(default))]
    fit_ids: Vec<F>,
}
impl<F> Default for FleetAddCmdGen<F> {
    fn default() -> Self {
        Self {
            fit_ids: Default::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F> FleetAddCmdGen<F> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit_ids(mut self, fit_ids: impl IntoIterator<Item = F>) -> Self {
        self.fit_ids.extend(fit_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetAddCmdBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<FleetAddCmd, BrResolveError> {
        Ok(FleetAddCmd {
            fit_ids: resps.resolve_fit_ids(self.fit_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetAddCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<AddedFleetIdResp, FleetAddError> {
        let mut core_fleet = core_sol.add_fleet();
        for fit_id in &self.fit_ids {
            match core_fleet.add_fit(fit_id) {
                Ok(()) => (),
                // Silently accept failure due to fit already being in this fleet
                Err(rc::err::FleetFitAddError::FitAlreadyInThisFleet(..)) => (),
                Err(err) => return Err(err.into()),
            }
        }
        Ok(AddedFleetIdResp::from_core_fleet(core_fleet))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FleetAddError {
    #[error("failed to add fit to fleet")]
    FitAdd(#[from] rc::err::FleetFitAddError),
}
