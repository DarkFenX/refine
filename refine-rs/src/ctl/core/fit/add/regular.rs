use super::shared::FitAddCmdShared;
use crate::{AddedFitIdResp, DpsProfile, FitSecStatus, FleetId};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct FitAddCmd {
    pub(super) fleet_id: Option<FleetId> = None,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) shared: FitAddCmdShared = FitAddCmdShared { .. },
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitAddCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<AddedFitIdResp, AddFitError> {
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
pub enum AddFitError {
    #[error("failed to set fleet")]
    FleetSet(#[from] rc::err::SetFitFleetError),
}
