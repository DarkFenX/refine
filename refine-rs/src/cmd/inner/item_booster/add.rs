use crate::{
    AddedItemIdsResp, CmdResps, FitIdBackref,
    cmd::shared::{EffectModes, SideEffects},
    err::BackrefRenderError,
};

// Commands with full context
pub(in crate::cmd) struct ICmdBoosterAddFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdBoosterAddICtx,
}
pub(crate) struct ICmdBoosterAddFCtxRIds {
    pub(in crate::cmd) fit_id: rc::FitId,
    pub(in crate::cmd) ictx_cmd: ICmdBoosterAddICtx,
}

// Commands with incomplete context
pub(crate) struct ICmdBoosterAddICtx {
    pub(in crate::cmd) type_id: rc::ItemTypeId,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) side_effects: SideEffects = SideEffects::new(),
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdBoosterAddFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdBoosterAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdBoosterAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdBoosterAddFCtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddBoosterError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddBoosterError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
}

impl ICmdBoosterAddICtx {
    pub(in crate::cmd) fn execute(&self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_booster = core_fit.add_booster(self.type_id);
        if let Some(state) = self.state {
            core_booster.set_state(state);
        }
        self.side_effects.apply(&mut core_booster);
        self.effect_modes.apply(&mut core_booster);
        AddedItemIdsResp::from_core_booster(core_booster)
    }
}
