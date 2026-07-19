use crate::{
    AddedItemIdsResp, CmdResps, FitId, FitIdBackref, ItemTypeId, SkillLevel, cmd::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdSkillAddFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) ictx_cmd: ICmdSkillAddICtx,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdSkillAddFCtxRIds {
    pub(in crate::cmd) fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) ictx_cmd: ICmdSkillAddICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdSkillAddICtx {
    pub(in crate::cmd) type_id: ItemTypeId,
    pub(in crate::cmd) level: SkillLevel,
    pub(in crate::cmd) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSkillAddFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdSkillAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdSkillAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSkillAddFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddSkillError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddSkillError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
    #[error("{0}")]
    AddFailed(#[from] FitAddSkillError),
}

impl ICmdSkillAddICtx {
    pub(in crate::cmd) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, FitAddSkillError> {
        let mut core_skill = core_fit.add_skill(self.type_id, self.level)?;
        if let Some(state) = self.state {
            core_skill.set_state(state);
        }
        self.effect_modes.apply(&mut core_skill);
        Ok(AddedItemIdsResp::from_core_skill(core_skill))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitAddSkillError {
    #[error("failed to add skill: {0}")]
    SkillAddFailed(#[from] rc::err::AddSkillError),
}
