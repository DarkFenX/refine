use crate::{
    AddedItemIdsResp, CtlCmdResps, FitId, FitIdBackref, ItemTypeId, SkillLevel, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdSkillAddFCtxBIds {
    pub(in crate::ctl) fit_id: FitIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdSkillAddICtx,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdSkillAddFCtxRIds {
    pub(in crate::ctl) fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdSkillAddICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdSkillAddICtx {
    pub(in crate::ctl) type_id: ItemTypeId,
    pub(in crate::ctl) level: SkillLevel,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSkillAddFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdSkillAddFCtxRIds, BackrefRenderError> {
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
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddSkillError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddSkillError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
    #[error(transparent)]
    SkillAdd(rc::err::AddSkillError),
}
impl From<FitAddSkillError> for GetFitAddSkillError {
    fn from(err: FitAddSkillError) -> Self {
        match err {
            FitAddSkillError::SkillAdd(inner) => Self::SkillAdd(inner),
        }
    }
}

impl ICmdSkillAddICtx {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, FitAddSkillError> {
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
    #[error(transparent)]
    SkillAdd(#[from] rc::err::AddSkillError),
}
