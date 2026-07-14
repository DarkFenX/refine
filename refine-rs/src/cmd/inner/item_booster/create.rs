use crate::cmd::shared::{BackrefRenderError, CmdResps, CreatedItemIdsResp, EffectModes, FitIdBackref, SideEffects};

// Commands with full context
pub(in crate::cmd) struct ICmdBoosterCreateFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdBoosterCreateICtx,
}
pub(crate) struct ICmdBoosterCreateFCtxRIds {
    pub(in crate::cmd) fit_id: rc::FitId,
    pub(in crate::cmd) ictx_cmd: ICmdBoosterCreateICtx,
}

// Commands with incomplete context
pub(crate) struct ICmdBoosterCreateICtx {
    pub(in crate::cmd) type_id: rc::ItemTypeId,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) side_effects: SideEffects = SideEffects::new(),
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdBoosterCreateFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdBoosterCreateFCtxRIds, BackrefRenderError> {
        Ok(ICmdBoosterCreateFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdBoosterCreateFCtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<CreatedItemIdsResp, GetFitCreateBoosterError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitCreateBoosterError {
    #[error("{0}")]
    FitGetFailed(#[from] rc::err::GetFitError),
}

impl ICmdBoosterCreateICtx {
    pub(in crate::cmd) fn execute(&self, core_fit: &mut rc::FitMut) -> CreatedItemIdsResp {
        let mut core_booster = core_fit.create_booster(self.type_id);
        if let Some(state) = self.state {
            core_booster.set_state(state);
        }
        self.side_effects.apply(&mut core_booster);
        self.effect_modes.apply(&mut core_booster);
        CreatedItemIdsResp::from_core_booster(core_booster)
    }
}
