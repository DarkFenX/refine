use crate::{
    cmd::{change_item, HCmdResp},
    util::HExecResult,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddSkillCmd {
    type_id: rc::EItemId,
    level: rc::SkillLevel,
    state: Option<bool>,
}
impl HAddSkillCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> HExecResult<rc::SolSkillInfo> {
        let info = core_sol.add_skill(*fit_id, self.type_id, self.level, self.state.unwrap_or(true))?;
        Ok(info)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeSkillCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeSkillCmd,
}
impl HChangeSkillCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
