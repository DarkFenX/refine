use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{cmd::HFitIdResp, shared::HDpsProfile, util::HExecError};

#[serde_as]
#[derive(Default, Deserialize)]
pub(crate) struct HAddFitCmd {
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    fleet_id: Option<rc::FleetId>,
    sec_status: Option<f64>,
    rah_incoming_dps: Option<HDpsProfile>,
}
impl HAddFitCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HFitIdResp, HExecError> {
        let mut core_fit = core_sol.add_fit();
        if let Some(fleet_id) = self.fleet_id {
            core_fit.set_fleet(&fleet_id).map_err(|error| match error {
                rc::err::SetFitFleetError::FleetNotFound(e) => HExecError::FleetNotFoundSecondary(e),
            })?;
        }
        if let Some(sec_status) = self.sec_status {
            let core_sec_status = rc::FitSecStatus::from_f64_clamped(sec_status);
            core_fit.set_sec_status(core_sec_status);
        }
        if let Some(rah_incoming_dps) = self.rah_incoming_dps {
            core_fit.set_rah_incoming_dps(rah_incoming_dps.into_core());
        }
        Ok(HFitIdResp::from_core_fit(core_fit))
    }
}
