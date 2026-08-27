use crate::{
    AddedItemIdsResp, CmdResps, EffectId, EffectMode, FitId, FitIdBr, ItemTypeId, SkillLevel,
    ctl::core::shared::EffectModes, err::BrResolveError, shared::CmdResidue,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct SkillAddCmd {
    type_id: ItemTypeId,
    level: SkillLevel,
    state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
pub type SkillAddCmdCtxFit = SkillAddCmdCtxFitGen<FitId>;
pub type SkillAddCmdCtxFitBr = SkillAddCmdCtxFitGen<FitIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct SkillAddCmdCtxFitGen<F> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: SkillAddCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillAddCmd {
    pub fn new(type_id: ItemTypeId, level: SkillLevel) -> Self {
        Self { type_id, level, .. }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillAddCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> SkillAddCmdCtxFit {
        SkillAddCmdCtxFit { fit_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> SkillAddCmdCtxFitBr {
        SkillAddCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillAddCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<SkillAddCmdCtxFit, BrResolveError> {
        Ok(SkillAddCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillAddCmd {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}
impl SkillAddCmdCtxFit {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}
impl SkillAddCmdCtxFitBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}

impl SkillAddCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, SkillAddError> {
        let mut core_skill = core_fit.add_skill(self.type_id, self.level)?;
        if let Some(state) = self.state {
            core_skill.set_state(state);
        }
        self.effect_modes.apply(&mut core_skill);
        Ok(AddedItemIdsResp::from_core_skill(core_skill))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum SkillAddError {
    #[error(transparent)]
    SkillAdd(#[from] rc::err::SkillAddError),
}

impl SkillAddCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, FitGetSkillAddError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetSkillAddError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
    #[error(transparent)]
    SkillAdd(rc::err::SkillAddError),
}
impl From<SkillAddError> for FitGetSkillAddError {
    fn from(err: SkillAddError) -> Self {
        match err {
            SkillAddError::SkillAdd(inner) => Self::SkillAdd(inner),
        }
    }
}
