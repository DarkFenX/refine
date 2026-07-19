use crate::{
    CmdResps, DpsProfile, FitId, FitIdBackref, FitSecStatus, FleetId, FleetIdBackref, TriStateField,
    err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdFitChangeFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) ictx_cmd: ICmdFitChangeICtxBIds = ICmdFitChangeICtxBIds { .. },
}
pub(crate) struct ICmdFitChangeFCtxRIds {
    fit_id: FitId,
    ictx_cmd: ICmdFitChangeICtxRIds,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdFitChangeICtxBIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) shared: CmdFitChangeShared = CmdFitChangeShared { .. },
    pub(in crate::cmd) fleet_id: TriStateField<FleetIdBackref> = TriStateField::Absent,
}
pub(crate) struct ICmdFitChangeICtxRIds {
    shared: CmdFitChangeShared,
    fleet_id: TriStateField<FleetId>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct CmdFitChangeShared {
    pub(in crate::cmd) sec_status: Option<FitSecStatus> = None,
    pub(in crate::cmd) rah_incoming_dps: TriStateField<DpsProfile> = TriStateField::Absent,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFitChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFitChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdFitChangeFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdFitChangeICtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFitChangeICtxRIds, BackrefRenderError> {
        Ok(ICmdFitChangeICtxRIds {
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
impl ICmdFitChangeFCtxRIds {
    pub(in crate::cmd) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), GetFitChangeFitError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitChangeFitError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
    #[error("{0}")]
    ChangeFailed(#[from] FitChangeFitError),
}

impl ICmdFitChangeICtxRIds {
    pub(in crate::cmd) fn execute(self, core_fit: &mut rc::FitMut) -> Result<(), FitChangeFitError> {
        match self.fleet_id {
            TriStateField::Value(fleet_id) => core_fit.set_fleet(&fleet_id)?,
            TriStateField::None => match core_fit.unset_fleet() {
                Ok(_) => (),
                // We are fine if fleet was not set
                Err(rc::err::UnsetFitFleetError::FitHasNoFleet(_)) => (),
            },
            TriStateField::Absent => (),
        }
        if let Some(sec_status) = self.shared.sec_status {
            core_fit.set_sec_status(sec_status);
        }
        match self.shared.rah_incoming_dps {
            TriStateField::Value(rah_incoming_dps) => core_fit.set_rah_incoming_dps(rah_incoming_dps),
            TriStateField::None => match core_fit.remove_rah_incoming_dps() {
                Ok(_) => (),
                // We are fine if profile was not set
                Err(rc::err::RemoveFitRahIncomingDpsError::DpsProfileNotSet(_)) => (),
            },
            TriStateField::Absent => (),
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitChangeFitError {
    #[error("failed to set fleet: {0}")]
    FleetSetFailed(#[from] rc::err::SetFitFleetError),
}
