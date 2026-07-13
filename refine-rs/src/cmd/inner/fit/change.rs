use crate::{
    cmd::{BackrefRenderError, CmdResps, FitIdBackref, FleetIdBackref},
    util::TriStateField,
};

// Commands with full context
pub(in crate::cmd) struct CmdFitChangeFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: CmdFitChangeICtxBIds,
}
pub(crate) struct CmdFitChangeFCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: CmdFitChangeICtxRIds,
}

// Commands with incomplete context
#[derive(Default)]
pub(in crate::cmd) struct CmdFitChangeICtxBIds {
    pub(in crate::cmd) shared: CmdFitChangeShared,
    pub(in crate::cmd) fleet_id: TriStateField<FleetIdBackref>,
}
pub(crate) struct CmdFitChangeICtxRIds {
    shared: CmdFitChangeShared,
    fleet_id: TriStateField<rc::FleetId>,
}
#[derive(Default)]
pub(in crate::cmd) struct CmdFitChangeShared {
    pub(in crate::cmd) sec_status: Option<rc::FitSecStatus>,
    pub(in crate::cmd) rah_incoming_dps: TriStateField<rc::DpsProfile>,
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
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), GetFitChangeFitError> {
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

impl CmdFitChangeICtxRIds {
    pub(in crate::cmd) fn execute(&self, core_fit: &mut rc::FitMut) -> Result<(), FitChangeFitError> {
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
