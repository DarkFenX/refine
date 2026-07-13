use crate::cmd::{
    BackrefRenderError, CmdResp, CmdResps, FitChangeFitError, FleetIdBackref,
    basic::{CmdFitChangeICtxBIds, CmdFitChangeICtxRIds},
};

pub enum ChangeFitEnumCmd {
    // Fit
    ChangeFit(FitChangeFitCmd),
}

pub(crate) enum ChangeFitEnumCmdRIds {
    // Fit
    ChangeFit(CmdFitChangeICtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeFitEnumCmd {
    pub(crate) fn render(self, resps: &CmdResps) -> Result<ChangeFitEnumCmdRIds, BackrefRenderError> {
        Ok(match self {
            // Fit
            Self::ChangeFit(cmd) => ChangeFitEnumCmdRIds::ChangeFit(cmd.basic.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeFitEnumCmdRIds {
    pub(crate) fn execute(&self, core_fit: &mut rc::FitMut) -> Result<CmdResp, ChangeFitEnumError> {
        match self {
            // Fit
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_fit)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeFitEnumError {
    #[error("failed to change fleet: {0}")]
    FitChangeFailed(#[from] FitChangeFitError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - fit
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct FitChangeFitCmd {
    basic: CmdFitChangeICtxBIds,
}
impl FitChangeFitCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: Option<FleetIdBackref>) -> Self {
        self.basic.fleet_id = fleet_id.into();
        self
    }
    pub fn with_sec_status(mut self, sec_status: rc::FitSecStatus) -> Self {
        self.basic.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: Option<rc::DpsProfile>) -> Self {
        self.basic.shared.rah_incoming_dps = rah_incoming_dps.into();
        self
    }
}
impl From<FitChangeFitCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeFitCmd) -> Self {
        Self::ChangeFit(sub_cmd)
    }
}
