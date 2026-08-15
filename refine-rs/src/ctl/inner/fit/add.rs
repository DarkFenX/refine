use crate::{AddedFitIdResp, CtlCmdResps, DpsProfile, FitSecStatus, FleetId, FleetIdBackref, err::BackrefRenderError};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdFitAddFCtxBIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) shared: ICmdFitAddShared = ICmdFitAddShared { .. },
    pub(in crate::ctl) fleet_id: Option<FleetIdBackref> = None,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdFitAddFCtxRIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) shared: ICmdFitAddShared = ICmdFitAddShared { .. },
    pub(in crate::ctl) fleet_id: Option<FleetId> = None,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdFitAddShared {
    pub(in crate::ctl) sec_status: Option<FitSecStatus> = None,
    pub(in crate::ctl) rah_incoming_dps: Option<DpsProfile> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFitAddFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdFitAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdFitAddFCtxRIds {
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
impl ICmdFitAddFCtxRIds {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<AddedFitIdResp, AddFitError> {
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
