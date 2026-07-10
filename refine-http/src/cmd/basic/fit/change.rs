use serde::Deserialize;

use crate::{
    cmd::shared::{HCmdResps, HFitIdBackref, HFleetIdBackref, get_primary_fit},
    err::HExecError,
    shared::HDpsProfile,
    util::TriStateField,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HFitChangeCmdFCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HFitChangeCmdICtxBIds,
}
pub(crate) struct HFitChangeCmdFCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: HFitChangeCmdICtxRIds,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HFitChangeCmdICtxBIds {
    #[serde(flatten)]
    shared: HFitChangeCmdShared,
    #[serde(default)]
    fleet_id: TriStateField<HFleetIdBackref>,
}
pub(crate) struct HFitChangeCmdICtxRIds {
    shared: HFitChangeCmdShared,
    fleet_id: TriStateField<rc::FleetId>,
}
#[derive(Deserialize)]
struct HFitChangeCmdShared {
    sec_status: Option<f64>,
    #[serde(default)]
    rah_incoming_dps: TriStateField<HDpsProfile>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFitChangeCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HFitChangeCmdFCtxRIds, HExecError> {
        Ok(HFitChangeCmdFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl HFitChangeCmdICtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HFitChangeCmdICtxRIds, HExecError> {
        Ok(HFitChangeCmdICtxRIds {
            shared: self.shared,
            fleet_id: match self.fleet_id {
                TriStateField::Value(fleet_id) => TriStateField::Value(resps.render_fleet_id(fleet_id)?),
                TriStateField::None => TriStateField::None,
                TriStateField::Absent => TriStateField::Absent,
            },
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFitChangeCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fit_id)
    }
}

impl HFitChangeCmdICtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<(), HExecError> {
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
        if let Some(sec_status) = self.shared.sec_status {
            let core_sec_status = rc::FitSecStatus::from_f64_clamped(sec_status);
            core_fit.set_sec_status(core_sec_status);
        }
        match self.shared.rah_incoming_dps {
            TriStateField::Value(rah_incoming_dps) => core_fit.set_rah_incoming_dps(rah_incoming_dps.into_core()),
            TriStateField::None => {
                // Do nothing if profile was not set
                let _ = core_fit.remove_rah_incoming_dps();
            }
            TriStateField::Absent => (),
        }
        Ok(())
    }
}
