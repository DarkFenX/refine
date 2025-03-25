use crate::{
    cmd::{
        HCmdResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeSkillCmd {
    level: Option<rc::SkillLevel>,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeSkillCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HCmdResp, HExecError> {
        if let Some(level) = self.level {
            if let Err(error) = core_sol.set_skill_level(item_id, level) {
                return Err(match error {
                    rc::err::SetSkillLevelError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::SetSkillLevelError::ItemIsNotSkill(e) => HExecError::ItemKindMismatch(e),
                    rc::err::SetSkillLevelError::SkillLevelError(e) => HExecError::InvalidSkillLevel(e),
                });
            }
        }
        if let Some(state) = self.state {
            if let Err(error) = core_sol.set_skill_state(item_id, state) {
                return Err(match error {
                    rc::err::SetSkillStateError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::SetSkillStateError::ItemIsNotSkill(e) => HExecError::ItemKindMismatch(e),
                });
            }
        }
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
