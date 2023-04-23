use crate::bridge::CmdResp;

use super::FitInfo;

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct FitCmdResp {
    fit: FitInfo,
    cmd_results: Vec<CmdResp>,
}
impl FitCmdResp {
    pub(crate) fn new(fit: FitInfo, cmd_results: Vec<CmdResp>) -> Self {
        Self { fit, cmd_results }
    }
}
