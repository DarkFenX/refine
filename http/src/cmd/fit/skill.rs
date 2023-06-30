use crate::cmd::{item, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HAddSkillCmd {
    type_id: rc::EItemId,
    level: rc::SkillLevel,
    state: Option<bool>,
}
impl HAddSkillCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem, fit_id: &rc::SsFitId) -> rc::Result<HCmdResp> {
        Ok(core_ss
            .add_skill(*fit_id, self.type_id, self.level, self.state.unwrap_or(true))?
            .into())
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeSkillCmd {
    #[serde(with = "crate::util::serde_string")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: item::HChangeSkillCmd,
}
impl HChangeSkillCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.item_cmd.execute(core_ss, &self.item_id)
    }
}
