use crate::cmd::{AddedItemIdsResp, BackrefRenderError, CmdResps, FitIdBackref, shared::EffectModes};

// Commands with full context
pub(in crate::cmd) struct ICmdImplantAddFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdImplantAddICtx,
}
pub(crate) struct ICmdImplantAddFCtxRIds {
    pub(in crate::cmd) fit_id: rc::FitId,
    pub(in crate::cmd) ictx_cmd: ICmdImplantAddICtx,
}

// Commands with incomplete context
pub(crate) struct ICmdImplantAddICtx {
    pub(in crate::cmd) type_id: rc::ItemTypeId,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdImplantAddFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdImplantAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdImplantAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdImplantAddFCtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddImplantError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddImplantError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
}

impl ICmdImplantAddICtx {
    pub(in crate::cmd) fn execute(&self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_implant = core_fit.add_implant(self.type_id);
        if let Some(state) = self.state {
            core_implant.set_state(state);
        }
        self.effect_modes.apply(&mut core_implant);
        AddedItemIdsResp::from_core_implant(core_implant)
    }
}
