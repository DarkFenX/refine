use crate::{CmdResps, FitId, FitIdBr, FleetId, FleetIdBr, err::BrResolveError};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FleetChangeCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    add_fit_ids: Vec<FitId>,
    #[cfg_attr(feature = "serde", serde(default))]
    rm_fit_ids: Vec<FitId>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FleetChangeCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    add_fit_ids: Vec<FitIdBr>,
    #[cfg_attr(feature = "serde", serde(default))]
    rm_fit_ids: Vec<FitIdBr>,
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FleetChangeCmdCtxFleet {
    fleet_id: FleetId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FleetChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FleetChangeCmdCtxFleetBr {
    fleet_id: FleetIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FleetChangeCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetChangeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_add_fit_ids(mut self, add_fit_ids: impl Iterator<Item = FitId>) -> Self {
        self.add_fit_ids.extend(add_fit_ids);
        self
    }
    pub fn with_rm_fit_ids(mut self, rm_fit_ids: impl Iterator<Item = FitId>) -> Self {
        self.rm_fit_ids.extend(rm_fit_ids);
        self
    }
}

impl FleetChangeCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_add_fit_ids(mut self, add_fit_ids: impl Iterator<Item = FitIdBr>) -> Self {
        self.add_fit_ids.extend(add_fit_ids);
        self
    }
    pub fn with_rm_fit_ids(mut self, rm_fit_ids: impl Iterator<Item = FitIdBr>) -> Self {
        self.rm_fit_ids.extend(rm_fit_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetChangeCmd {
    pub(in crate::ctl) fn into_ctx_fleet(self, fleet_id: FleetId) -> FleetChangeCmdCtxFleet {
        FleetChangeCmdCtxFleet { fleet_id, core: self }
    }
}

impl FleetChangeCmdBr {
    pub(in crate::ctl) fn into_ctx_fleet_br(self, fleet_id: impl Into<FleetIdBr>) -> FleetChangeCmdCtxFleetBr {
        FleetChangeCmdCtxFleetBr {
            fleet_id: fleet_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetChangeCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<FleetChangeCmd, BrResolveError> {
        Ok(FleetChangeCmd {
            add_fit_ids: resps.resolve_fit_ids(self.add_fit_ids)?,
            rm_fit_ids: resps.resolve_fit_ids(self.rm_fit_ids)?,
        })
    }
}

impl FleetChangeCmdCtxFleetBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<FleetChangeCmdCtxFleet, BrResolveError> {
        Ok(FleetChangeCmdCtxFleet {
            fleet_id: resps.resolve_fleet_id(self.fleet_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetChangeCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> Result<(), FleetChangeError> {
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
pub enum FleetChangeError {
    #[error("failed to add fit")]
    FitAdd(#[from] rc::err::FleetFitAddError),
    #[error("failed to remove fit")]
    FitRemove(#[from] rc::err::FleetFitRemoveError),
}

impl FleetChangeCmdCtxFleet {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), FleetGetFleetChangeError> {
        let mut core_fleet = core_sol.get_fleet_mut(&self.fleet_id)?;
        Ok(self.core.execute(&mut core_fleet)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FleetGetFleetChangeError {
    #[error(transparent)]
    FleetGet(#[from] rc::err::FleetGetError),
    #[error("failed to add fit")]
    FitAdd(#[source] rc::err::FleetFitAddError),
    #[error("failed to remove fit")]
    FitRemove(#[source] rc::err::FleetFitRemoveError),
}
impl From<FleetChangeError> for FleetGetFleetChangeError {
    fn from(err: FleetChangeError) -> Self {
        match err {
            FleetChangeError::FitAdd(inner) => Self::FitAdd(inner),
            FleetChangeError::FitRemove(inner) => Self::FitRemove(inner),
        }
    }
}
