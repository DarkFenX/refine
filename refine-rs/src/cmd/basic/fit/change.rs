use crate::{
    cmd::{BackrefRenderError, CmdResps, FitIdBackref, FleetIdBackref},
    util::TriStateField,
};

// Commands with full context
struct CmdFitChangeFCtxBIds {
    fit_id: FitIdBackref,
    ictx_cmd: CmdFitChangeICtxBIds,
}
struct CmdFitChangeFCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: CmdFitChangeICtxRIds,
}

// Commands with incomplete context
struct CmdFitChangeICtxBIds {
    shared: CmdFitChangeShared,
    fleet_id: TriStateField<FleetIdBackref>,
}
struct CmdFitChangeICtxRIds {
    shared: CmdFitChangeShared,
    fleet_id: TriStateField<rc::FleetId>,
}
struct CmdFitChangeShared {
    sec_status: Option<rc::FitSecStatus>,
    rah_incoming_dps: TriStateField<rc::DpsProfile>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdFitChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<CmdFitChangeFCtxRIds, BackrefRenderError> {
        Ok(CmdFitChangeFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl CmdFitChangeICtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<CmdFitChangeICtxRIds, BackrefRenderError> {
        Ok(CmdFitChangeICtxRIds {
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
impl CmdFitChangeFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), GetChangeFitError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetChangeFitError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
    #[error("{0}")]
    ChangeFailed(#[from] ChangeFitError),
}

impl CmdFitChangeICtxRIds {
    pub(in crate::cmd) fn execute(&self, core_fit: &mut rc::FitMut) -> Result<(), ChangeFitError> {
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
pub enum ChangeFitError {
    #[error("failed to set fleet: {0}")]
    FleetSetFailed(#[from] rc::err::SetFitFleetError),
}
