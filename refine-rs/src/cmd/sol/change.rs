use crate::cmd::{
    BackrefRenderError, CmdResp, CmdResps, FitIdBackref, FleetIdBackref, GetFitChangeFitError,
    basic::{CmdFitChangeFCtxBIds, CmdFitChangeFCtxRIds, CmdFitChangeICtxBIds},
};

pub enum ChangeSolEnumCmd {
    // Fit
    ChangeFit(SolChangeFitCmd),
}

pub(crate) enum ChangeSolEnumCmdRIds {
    // Fit
    ChangeFit(CmdFitChangeFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeSolEnumCmd {
    pub(crate) fn render(self, resps: &CmdResps) -> Result<ChangeSolEnumCmdRIds, BackrefRenderError> {
        Ok(match self {
            // Fit
            Self::ChangeFit(cmd) => ChangeSolEnumCmdRIds::ChangeFit(cmd.basic.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeSolEnumCmdRIds {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<CmdResp, ChangeSolEnumError> {
        match self {
            // Fit
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeSolEnumError {
    #[error("failed to change fleet: {0}")]
    FitChangeFailed(#[from] GetFitChangeFitError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - fit
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolChangeFitCmd {
    basic: CmdFitChangeFCtxBIds,
}
impl SolChangeFitCmd {
    pub fn new(fit_id: FitIdBackref) -> Self {
        Self {
            basic: CmdFitChangeFCtxBIds {
                fit_id,
                ictx_cmd: CmdFitChangeICtxBIds::default(),
            },
        }
    }
    pub fn with_fleet_id(mut self, fleet_id: Option<FleetIdBackref>) -> Self {
        self.basic.ictx_cmd.fleet_id = fleet_id.into();
        self
    }
    pub fn with_sec_status(mut self, sec_status: rc::FitSecStatus) -> Self {
        self.basic.ictx_cmd.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: Option<rc::DpsProfile>) -> Self {
        self.basic.ictx_cmd.shared.rah_incoming_dps = rah_incoming_dps.into();
        self
    }
}
impl From<SolChangeFitCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeFitCmd) -> Self {
        Self::ChangeFit(sub_cmd)
    }
}
