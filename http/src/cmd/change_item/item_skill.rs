use serde::Deserialize;

use crate::{
    cmd::{HItemIdsResp, shared::HEffectModeMap},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HChangeSkillCmd {
    type_id: Option<i32>,
    level: Option<i32>,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeSkillCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_skill = core_sol.get_skill_mut(item_id).map_err(|error| match error {
            rc::err::GetSkillError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetSkillError::ItemIsNotSkill(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_skill.set_type_id(core_type_id).map_err(|error| match error {
                rc::err::SetSkillTypeIdError::SkillIdCollision(e) => HExecError::SkillIdCollision(e),
            })?;
        }
        if let Some(level) = self.level {
            let core_level = rc::SkillLevel::from_i32_clamped(level);
            core_skill.set_level(core_level);
        }
        if let Some(state) = self.state {
            core_skill.set_state(state);
        }
        if let Some(effect_modes) = self.effect_modes.as_ref() {
            effect_modes.apply(&mut core_skill);
        }
        Ok(HItemIdsResp::from_core_skill(core_skill))
    }
}
