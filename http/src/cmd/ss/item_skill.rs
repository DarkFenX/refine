use crate::cmd::{fit, HCmdResp};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HAddSkillCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HAddSkillCmd,
}
impl HAddSkillCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeSkillCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeSkillCmd,
}
impl HChangeSkillCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss)
    }
}
