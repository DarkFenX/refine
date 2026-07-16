use crate::{AddedItemIdsResp, CmdResps, FitIdBackref, cmd::shared::EffectModes, err::BackrefRenderError};

// Commands with full context
pub(in crate::cmd) struct ICmdFwEffectAddFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdFwEffectAddICtx,
}
pub(crate) struct ICmdFwEffectAddFCtxRIds {
    pub(in crate::cmd) fit_id: rc::FitId,
    pub(in crate::cmd) ictx_cmd: ICmdFwEffectAddICtx,
}

// Commands with incomplete context
pub(crate) struct ICmdFwEffectAddICtx {
    pub(in crate::cmd) type_id: rc::ItemTypeId,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFwEffectAddFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFwEffectAddFCtxRIds, BackrefRenderError> {
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
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddFwEffectError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddFwEffectError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
}

impl ICmdFwEffectAddICtx {
    pub(in crate::cmd) fn execute(self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_fw_effect = core_fit.add_fw_effect(self.type_id);
        if let Some(state) = self.state {
            core_fw_effect.set_state(state);
        }
        self.effect_modes.apply(&mut core_fw_effect);
        AddedItemIdsResp::from_core_fw_effect(core_fw_effect)
    }
}
