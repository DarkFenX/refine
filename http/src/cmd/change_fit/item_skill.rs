use crate::{
    cmd::{change_item, HCmdResp},
    util::HExecError,
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
    ) -> Result<rc::SolSkillInfo, HExecError> {
        let core_skill = match core_sol.add_skill(*fit_id, self.type_id, self.level, self.state.unwrap_or(true)) {
            Ok(core_skill) => core_skill,
            Err(error) => {
                return Err(match error {
                    rc::err::AddSkillError::InvalidSkillLevel(e) => HExecError::InvalidSkillLevel(e),
                    rc::err::AddSkillError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                })
            }
        };
        Ok(core_skill)
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
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
