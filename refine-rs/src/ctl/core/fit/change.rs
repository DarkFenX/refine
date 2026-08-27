use crate::{
    CmdResps, DpsProfile, FitId, FitIdBr, FitSecStatus, FleetId, FleetIdBr, TriStateField, err::BrResolveError,
    shared::CmdResidue,
};

// Core commands
pub type FitChangeCmd = FitChangeCmdGen<FleetId>;
pub type FitChangeCmdBr = FitChangeCmdGen<FleetIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "L: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FitChangeCmdGen<L> {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_id: TriStateField<L>,
    sec_status: Option<FitSecStatus>,
    #[cfg_attr(feature = "serde", serde(default))]
    rah_incoming_dps: TriStateField<DpsProfile>,
}
impl<L> Default for FitChangeCmdGen<L> {
    fn default() -> Self {
        Self {
            fleet_id: Default::default(),
            sec_status: Default::default(),
            rah_incoming_dps: Default::default(),
        }
    }
}

// Extra context commands
pub type FitChangeCmdCtxFit = FitChangeCmdCtxFitGen<FleetId, FitId>;
pub type FitChangeCmdCtxFitBr = FitChangeCmdCtxFitGen<FleetIdBr, FitIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "L: serde::Deserialize<'de>, F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FitChangeCmdCtxFitGen<L, F> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitChangeCmdGen<L>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<L> FitChangeCmdGen<L> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: Option<L>) -> Self {
        self.fleet_id = fleet_id.into();
        self
    }
    pub fn with_sec_status(mut self, sec_status: FitSecStatus) -> Self {
        self.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: Option<DpsProfile>) -> Self {
        self.rah_incoming_dps = rah_incoming_dps.into();
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitChangeCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> FitChangeCmdCtxFit {
        FitChangeCmdCtxFit { fit_id, core: self }
    }
}
impl FitChangeCmdBr {
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> FitChangeCmdCtxFitBr {
        FitChangeCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitChangeCmdBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<FitChangeCmd, BrResolveError> {
        Ok(FitChangeCmd {
            sec_status: self.sec_status,
            rah_incoming_dps: self.rah_incoming_dps,
            fleet_id: match self.fleet_id {
                TriStateField::Value(fleet_id) => TriStateField::Value(resps.resolve_fleet_id(fleet_id)?),
                TriStateField::None => TriStateField::None,
                TriStateField::Absent => TriStateField::Absent,
            },
        })
    }
}

impl FitChangeCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<FitChangeCmdCtxFit, BrResolveError> {
        Ok(FitChangeCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<L> FitChangeCmdGen<L> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        // Assume the command always mutates (even if it does not with none of fields set)
        // Fleet change goes first, so its fail is clean
        if self.fleet_id.is_value() {
            return CmdResidue::MutFallibleClean;
        }
        CmdResidue::MutInfallible
    }
}
impl<L, F> FitChangeCmdCtxFitGen<L, F> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        // This one can fail regardless of core command contents
        CmdResidue::MutFallibleClean
    }
}

impl FitChangeCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<(), FitChangeError> {
        match self.fleet_id {
            TriStateField::Value(fleet_id) => core_fit.set_fleet(&fleet_id)?,
            TriStateField::None => match core_fit.unset_fleet() {
                Ok(..) => (),
                // We are fine if fleet was not set
                Err(rc::err::FitFleetUnsetError::FitHasNoFleet(..)) => (),
            },
            TriStateField::Absent => (),
        }
        if let Some(sec_status) = self.sec_status {
            core_fit.set_sec_status(sec_status);
        }
        match self.rah_incoming_dps {
            TriStateField::Value(rah_incoming_dps) => core_fit.set_rah_incoming_dps(rah_incoming_dps),
            TriStateField::None => match core_fit.remove_rah_incoming_dps() {
                Ok(..) => (),
                // We are fine if profile was not set
                Err(rc::err::FitRahIncomingDpsRemoveError::DpsProfileNotSet(..)) => (),
            },
            TriStateField::Absent => (),
        }
        Ok(())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitChangeError {
    #[error("failed to set fleet")]
    FleetSet(#[from] rc::err::FitFleetSetError),
}

impl FitChangeCmdCtxFit {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), FitGetFitChangeError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetFitChangeError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
    #[error("failed to set fleet")]
    FleetSet(#[source] rc::err::FitFleetSetError),
}
impl From<FitChangeError> for FitGetFitChangeError {
    fn from(err: FitChangeError) -> Self {
        match err {
            FitChangeError::FleetSet(inner) => Self::FleetSet(inner),
        }
    }
}
