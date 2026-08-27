use crate::{
    AddedFitIdResp, CmdResps, DpsProfile, FitSecStatus, FleetId, FleetIdBr, err::BrResolveError, shared::CmdResidue,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitAddCmd {
    fleet_id: Option<FleetId>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitAddCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitAddCmdBr {
    fleet_id: Option<FleetIdBr>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitAddCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
struct FitAddCmdShared {
    sec_status: Option<FitSecStatus>,
    rah_incoming_dps: Option<DpsProfile>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitAddCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: FleetId) -> Self {
        self.fleet_id = Some(fleet_id);
        self
    }
    pub fn with_sec_status(mut self, sec_status: FitSecStatus) -> Self {
        self.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: DpsProfile) -> Self {
        self.shared.rah_incoming_dps = Some(rah_incoming_dps);
        self
    }
}

impl FitAddCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: FleetIdBr) -> Self {
        self.fleet_id = Some(fleet_id);
        self
    }
    pub fn with_sec_status(mut self, sec_status: FitSecStatus) -> Self {
        self.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: DpsProfile) -> Self {
        self.shared.rah_incoming_dps = Some(rah_incoming_dps);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitAddCmdBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<FitAddCmd, BrResolveError> {
        Ok(FitAddCmd {
            shared: self.shared,
            fleet_id: match self.fleet_id {
                Some(fleet_id) => Some(resps.resolve_fleet_id(fleet_id)?),
                None => None,
            },
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitAddCmdBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self.fleet_id {
            Some(..) => CmdResidue::MutFallibleDirty,
            None => CmdResidue::MutInfallible,
        }
    }
}

impl FitAddCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<AddedFitIdResp, FitAddError> {
        let mut core_fit = core_sol.add_fit();
        if let Some(fleet_id) = self.fleet_id {
            core_fit.set_fleet(&fleet_id)?;
        }
        if let Some(sec_status) = self.shared.sec_status {
            core_fit.set_sec_status(sec_status);
        }
        if let Some(rah_incoming_dps) = self.shared.rah_incoming_dps {
            core_fit.set_rah_incoming_dps(rah_incoming_dps);
        }
        Ok(AddedFitIdResp::from_core_fit(core_fit))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitAddError {
    #[error("failed to set fleet")]
    FleetSet(#[from] rc::err::FitFleetSetError),
}
