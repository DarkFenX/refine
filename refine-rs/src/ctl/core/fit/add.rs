use crate::{AddedFitIdResp, CmdResps, DpsProfile, FitSecStatus, FleetId, FleetIdBr, err::BackrefRenderError};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct FitAddCmd {
    fleet_id: Option<FleetId>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitAddCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct FitAddCmdBr {
    fleet_id: Option<FleetIdBr>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitAddCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
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
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitAddCmd {
    pub(in crate::ctl) fn into_br(self) -> FitAddCmdBr {
        FitAddCmdBr {
            fleet_id: self.fleet_id.map(FleetIdBr::Id),
            shared: self.shared,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitAddCmdBr {
    pub(in crate::ctl) fn render(self, resps: &CmdResps) -> Result<FitAddCmd, BackrefRenderError> {
        Ok(FitAddCmd {
            shared: self.shared,
            fleet_id: match self.fleet_id {
                Some(fleet_id) => Some(resps.render_fleet_id(fleet_id)?),
                None => None,
            },
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
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
    FleetSet(#[from] rc::err::SetFitFleetError),
}
