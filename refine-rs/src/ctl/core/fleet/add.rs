use crate::{AddedFleetIdResp, CmdResps, FitId, FitIdBr, err::BrResolveError};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FleetAddCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fit_ids: Vec<FitId>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FleetAddCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    fit_ids: Vec<FitIdBr>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetAddCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit_ids(mut self, fit_ids: impl Iterator<Item = FitId>) -> Self {
        self.fit_ids.extend(fit_ids);
        self
    }
}

impl FleetAddCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit_ids(mut self, fit_ids: impl Iterator<Item = FitIdBr>) -> Self {
        self.fit_ids.extend(fit_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetAddCmd {
    pub(in crate::ctl) fn into_br(self) -> FleetAddCmdBr {
        FleetAddCmdBr {
            fit_ids: self.fit_ids.into_iter().map(FitIdBr::Id).collect(),
        }
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
            core_fleet.add_fit(fit_id)?;
        }
        Ok(AddedFleetIdResp::from_core_fleet(core_fleet))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FleetAddError {
    #[error("failed to add fit to fleet")]
    FitAdd(#[from] rc::err::FleetAddFitError),
}
