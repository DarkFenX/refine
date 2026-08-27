use crate::{CmdResps, FitId, FitIdBr, FleetId, FleetIdBr, err::BrResolveError, shared::CmdResidue};

// Core commands
pub type FleetChangeCmd = FleetChangeCmdGen<FitId>;
pub type FleetChangeCmdBr = FleetChangeCmdGen<FitIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FleetChangeCmdGen<F> {
    #[cfg_attr(feature = "serde", serde(default))]
    add_fit_ids: Vec<F>,
    #[cfg_attr(feature = "serde", serde(default))]
    rm_fit_ids: Vec<F>,
}
impl<F> Default for FleetChangeCmdGen<F> {
    fn default() -> Self {
        Self {
            add_fit_ids: Default::default(),
            rm_fit_ids: Default::default(),
        }
    }
}

// Extra context commands
pub type FleetChangeCmdCtxFleet = FleetChangeCmdCtxFleetGen<FleetId, FitId>;
pub type FleetChangeCmdCtxFleetBr = FleetChangeCmdCtxFleetGen<FleetIdBr, FitIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "L: serde::Deserialize<'de>, F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FleetChangeCmdCtxFleetGen<L, F> {
    fleet_id: L,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FleetChangeCmdGen<F>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F> FleetChangeCmdGen<F> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_add_fit_ids(mut self, add_fit_ids: impl Iterator<Item = F>) -> Self {
        self.add_fit_ids.extend(add_fit_ids);
        self
    }
    pub fn with_rm_fit_ids(mut self, rm_fit_ids: impl Iterator<Item = F>) -> Self {
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
impl<F> FleetChangeCmdGen<F> {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        // Assume the command always mutates (even if it does not with none of fields set)
        match (self.rm_fit_ids.len(), self.add_fit_ids.len()) {
            (0, 0) => CmdResidue::MutInfallible,
            (1, 0) | (0, 1) => CmdResidue::MutFallibleClean,
            _ => CmdResidue::MutFallibleDirty,
        }
    }
}
impl<L, F> FleetChangeCmdCtxFleetGen<L, F> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        match self.core.exec_residue() {
            CmdResidue::MutInfallible => CmdResidue::MutFallibleClean,
            n => n,
        }
    }
}

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
