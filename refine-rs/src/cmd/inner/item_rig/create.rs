use crate::cmd::{BackrefRenderError, CmdResps, CreatedItemIdsResp, FitIdBackref, shared::EffectModes};

// Commands with full context
struct CmdRigCreateFCtxBIds {
    fit_id: FitIdBackref,
    ictx_cmd: CmdRigCreateICtx,
}
struct CmdRigCreateFCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: CmdRigCreateICtx,
}

// Commands with incomplete context
struct CmdRigCreateICtx {
    type_id: rc::ItemTypeId,
    state: Option<bool>,
    effect_modes: EffectModes,
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
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<CreatedItemIdsResp, CreateRigError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateRigError {
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
