use crate::cmd::{fit, HCmdResp};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HAddFighterCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HAddFighterCmd,
}
impl HAddFighterCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeFighterCmd,
}
impl HChangeFighterCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss)
    }
}
