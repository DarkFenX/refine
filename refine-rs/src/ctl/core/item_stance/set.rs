use crate::{
    AddedItemIdsResp, CmdResps, EffectId, EffectMode, FitId, FitIdBr, ItemTypeId, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct StanceSetCmd {
    type_id: ItemTypeId,
    state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct StanceSetCmdCtxFit {
    fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: StanceSetCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct StanceSetCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: StanceSetCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StanceSetCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self { type_id, .. }
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
impl StanceSetCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> StanceSetCmdCtxFit {
        StanceSetCmdCtxFit { fit_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> StanceSetCmdCtxFitBr {
        StanceSetCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StanceSetCmdCtxFitBr {
    pub(in crate::ctl) fn render(self, resps: &CmdResps) -> Result<StanceSetCmdCtxFit, BackrefRenderError> {
        Ok(StanceSetCmdCtxFit {
            fit_id: resps.render_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StanceSetCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_stance = core_fit.set_stance(self.type_id);
        if let Some(state) = self.state {
            core_stance.set_state(state);
        }
        self.effect_modes.apply(&mut core_stance);
        AddedItemIdsResp::from_core_stance(core_stance)
    }
}

impl StanceSetCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, FitGetStanceSetError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetStanceSetError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
}
