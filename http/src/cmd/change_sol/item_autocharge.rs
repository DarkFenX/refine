use serde::Deserialize;

use crate::{
    cmd::{HItemIdsResp, change_fit},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HChangeAutochargeCmd {
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeAutochargeCmd,
}
impl HChangeAutochargeCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.fit_cmd.execute(core_sol)
    }
}
