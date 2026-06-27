use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::shared::{HCmdResps, HCreatedItemIdsResp, HEffectModeMap, HFitIdBackref, get_primary_fit},
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HSkillAddCmdFCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HSkillAddCmdICtx,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HSkillAddCmdFCtxRIds {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    ictx_cmd: HSkillAddCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HSkillAddCmdICtx {
    type_id: i32,
    level: i32,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSkillAddCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HSkillAddCmdFCtxRIds, HExecError> {
        Ok(HSkillAddCmdFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSkillAddCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCreatedItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fit_id)
    }
}

impl HSkillAddCmdICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCreatedItemIdsResp, HExecError> {
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
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_skill);
        }
        Ok(HCreatedItemIdsResp::from_core_skill(core_skill))
    }
}
