use crate::cmd::{fit, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeChargeCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeChargeCmd,
}
impl HChangeChargeCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss)
    }
}
