use crate::{
    cmd::{change_fit, HCmdResp},
    util::HExecResult,
};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeChargeCmd {
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeChargeCmd,
}
impl HChangeChargeCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        self.fit_cmd.execute(core_sol)
    }
}
