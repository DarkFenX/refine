use crate::cmd::{BackrefRenderError, CmdResps, CreatedItemIdsResp, FitIdBackref, shared::EffectModes};

// Commands with full context
pub(in crate::cmd) struct CmdRigCreateFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: CmdRigCreateICtx,
}
pub(crate) struct CmdRigCreateFCtxRIds {
    pub(in crate::cmd) fit_id: rc::FitId,
    pub(in crate::cmd) ictx_cmd: CmdRigCreateICtx,
}

// Commands with incomplete context
pub(crate) struct CmdRigCreateICtx {
    pub(in crate::cmd) type_id: rc::ItemTypeId,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdRigCreateFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<CmdRigCreateFCtxRIds, BackrefRenderError> {
        Ok(CmdRigCreateFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdRigCreateFCtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<CreatedItemIdsResp, GetFitCreateRigError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitCreateRigError {
    #[error("{0}")]
    FitGetFailed(#[from] rc::err::GetFitError),
}

impl CmdRigCreateICtx {
    pub(in crate::cmd) fn execute(&self, core_fit: &mut rc::FitMut) -> CreatedItemIdsResp {
        let mut core_rig = core_fit.create_rig(self.type_id);
        if let Some(state) = self.state {
            core_rig.set_state(state);
        }
        self.effect_modes.apply(&mut core_rig);
        CreatedItemIdsResp::from_core_rig(core_rig)
    }
}
