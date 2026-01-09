use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{HItemIdsResp, change_item, shared::get_primary_fit},
    util::HExecError,
};
#[derive(Deserialize)]
pub(crate) struct HAddSkillCmd {
    type_id: i32,
    level: i32,
    state: Option<bool>,
}
impl HAddSkillCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        let core_level = rc::SkillLevel::from_i32_clamped(self.level);
        let mut core_skill = core_fit
            .add_skill(core_type_id, core_level)
            .map_err(|error| match error {
                rc::err::AddSkillError::SkillIdCollision(e) => HExecError::SkillIdCollision(e),
            })?;
        if let Some(state) = self.state {
            core_skill.set_state(state);
        }
        Ok(core_skill.into())
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeSkillCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeSkillCmd,
}
impl HChangeSkillCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
