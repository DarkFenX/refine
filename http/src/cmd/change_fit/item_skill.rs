use crate::{
    cmd::{HItemIdsResp, change_item, shared::get_primary_fit},
    shared::HSkillLevel,
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddSkillCmd {
    type_id: rc::ItemTypeId,
    level: HSkillLevel,
    state: Option<bool>,
}
impl HAddSkillCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let core_level = rc::SkillLevel::from_i32_clamped(self.level);
        let mut core_skill = core_fit
            .add_skill(self.type_id, core_level)
            .map_err(|error| match error {
                rc::err::AddSkillError::SkillIdCollision(e) => HExecError::SkillIdCollision(e),
            })?;
        if let Some(state) = self.state {
            core_skill.set_state(state);
        }
        Ok(core_skill.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeSkillCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeSkillCmd,
}
impl HChangeSkillCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
