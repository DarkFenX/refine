use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{HFitIdResp, shared::get_primary_fit},
    shared::HDpsProfile,
    util::{HExecError, TriStateField},
};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeFitCmd {
    #[serde_as(as = "TriStateField<DisplayFromStr>")]
    #[serde(default)]
    fleet_id: TriStateField<rc::FleetId>,
    sec_status: Option<f64>,
    #[serde(default)]
    rah_incoming_dps: TriStateField<HDpsProfile>,
}
impl HChangeFitCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HFitIdResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        match self.fleet_id {
            TriStateField::Value(fleet_id) => {
                core_fit.set_fleet(&fleet_id).map_err(|error| match error {
                    rc::err::SetFitFleetError::FleetNotFound(e) => HExecError::FleetNotFoundSecondary(e),
                })?;
            }
            TriStateField::None => {
                core_fit.unset_fleet().map_err(|error| match error {
                    rc::err::UnsetFitFleetError::FitHasNoFleet(e) => HExecError::FitNotInFleet(e),
                })?;
            }
            TriStateField::Absent => (),
        }
        if let Some(sec_status) = self.sec_status {
            let core_sec_status = rc::FitSecStatus::from_f64_clamped(sec_status);
            core_fit.set_sec_status(core_sec_status);
        }
        match self.rah_incoming_dps {
            TriStateField::Value(rah_incoming_dps) => core_fit.set_rah_incoming_dps(rah_incoming_dps.into_core()),
            TriStateField::None => {
                // Do nothing if profile was not set
                let _ = core_fit.remove_rah_incoming_dps();
            }
            TriStateField::Absent => (),
        }
        Ok(core_fit.into())
    }
}
