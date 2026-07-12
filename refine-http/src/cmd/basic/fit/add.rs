use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::shared::{HCmdResps, HCreatedFitIdResp, HFleetIdBackref},
    err::HExecError,
    shared::HDpsProfile,
};

// Commands with full context
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HFitCreateCmdFCtxBIds {
    #[serde(flatten)]
    shared: HFitCreateCmdShared,
    fleet_id: Option<HFleetIdBackref>,
}
#[serde_as]
#[derive(Default, Deserialize)]
pub(crate) struct HFitCreateCmdFCtxRIds {
    #[serde(flatten)]
    shared: HFitCreateCmdShared,
    #[serde_as(as = "Option<DisplayFromStr>")]
    fleet_id: Option<rc::FleetId>,
}
#[derive(Default, Deserialize)]
struct HFitCreateCmdShared {
    sec_status: Option<f64>,
    rah_incoming_dps: Option<HDpsProfile>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFitCreateCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HFitCreateCmdFCtxRIds, HExecError> {
        Ok(HFitCreateCmdFCtxRIds {
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
impl HFitCreateCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCreatedFitIdResp, HExecError> {
        let mut core_fit = core_sol.create_fit();
        if let Some(fleet_id) = self.fleet_id {
            core_fit.set_fleet(&fleet_id).map_err(|error| match error {
                rc::err::SetFitFleetError::FleetNotFound(e) => HExecError::FleetNotFoundSecondary(e),
            })?;
        }
        if let Some(sec_status) = self.shared.sec_status {
            let core_sec_status = rc::FitSecStatus::from_f64_clamped(sec_status);
            core_fit.set_sec_status(core_sec_status);
        }
        if let Some(rah_incoming_dps) = self.shared.rah_incoming_dps {
            core_fit.set_rah_incoming_dps(rah_incoming_dps.into_core());
        }
        Ok(HCreatedFitIdResp::from_core_fit(core_fit))
    }
}
