use crate::{
    AddedItemIdsResp, CtlCmdResps, FitId, FitIdBr, ItemTypeId, ctl::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdFwEffectAddFCtxBIds {
    pub(in crate::ctl) fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdFwEffectAddICtx,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdFwEffectAddFCtxRIds {
    pub(in crate::ctl) fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdFwEffectAddICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdFwEffectAddICtx {
    pub(in crate::ctl) type_id: ItemTypeId,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFwEffectAddFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdFwEffectAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdFwEffectAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFwEffectAddFCtxRIds {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddFwEffectError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddFwEffectError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
}

impl ICmdFwEffectAddICtx {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_fw_effect = core_fit.add_fw_effect(self.type_id);
        if let Some(state) = self.state {
            core_fw_effect.set_state(state);
        }
        self.effect_modes.apply(&mut core_fw_effect);
        AddedItemIdsResp::from_core_fw_effect(core_fw_effect)
    }
}
