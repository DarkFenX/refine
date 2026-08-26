use crate::{
    CmdResps, DpsProfile, FitId, FitIdBr, FitSecStatus, FleetId, FleetIdBr, TriStateField, err::BrResolveError,
    shared::CmdResidue,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitChangeCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_id: TriStateField<FleetId>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: CmdFitChangeShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitChangeCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_id: TriStateField<FleetIdBr>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: CmdFitChangeShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
struct CmdFitChangeShared {
    sec_status: Option<FitSecStatus>,
    #[cfg_attr(feature = "serde", serde(default))]
    rah_incoming_dps: TriStateField<DpsProfile>,
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FitChangeCmdCtxFit {
    fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FitChangeCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitChangeCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitChangeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: Option<FleetId>) -> Self {
        self.fleet_id = fleet_id.into();
        self
    }
    pub fn with_sec_status(mut self, sec_status: FitSecStatus) -> Self {
        self.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: Option<DpsProfile>) -> Self {
        self.shared.rah_incoming_dps = rah_incoming_dps.into();
        self
    }
}

impl FitChangeCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: Option<FleetIdBr>) -> Self {
        self.fleet_id = fleet_id.into();
        self
    }
    pub fn with_sec_status(mut self, sec_status: FitSecStatus) -> Self {
        self.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: Option<DpsProfile>) -> Self {
        self.shared.rah_incoming_dps = rah_incoming_dps.into();
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
            shared: self.shared,
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
impl FitChangeCmdBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self.fleet_id {
            TriStateField::Value(..) => CmdResidue::FallibleClean,
            _ => CmdResidue::Infallible,
        }
    }
}
impl FitChangeCmdCtxFitBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        self.core.exec_residue()
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
        if let Some(sec_status) = self.shared.sec_status {
            core_fit.set_sec_status(sec_status);
        }
        match self.shared.rah_incoming_dps {
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
