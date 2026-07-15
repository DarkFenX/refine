use crate::cmd::{AddedItemIdsResp, BackrefRenderError, CmdResps, FitIdBackref, shared::EffectModes};

// Commands with full context
pub(in crate::cmd) struct ICmdSubsystemAddFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdSubsystemAddICtx,
}
pub(crate) struct ICmdSubsystemAddFCtxRIds {
    pub(in crate::cmd) fit_id: rc::FitId,
    pub(in crate::cmd) ictx_cmd: ICmdSubsystemAddICtx,
}

// Commands with incomplete context
pub(crate) struct ICmdSubsystemAddICtx {
    pub(in crate::cmd) type_id: rc::ItemTypeId,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSubsystemAddFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdSubsystemAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdSubsystemAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSubsystemAddFCtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddSubsystemError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddSubsystemError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
}

impl ICmdSubsystemAddICtx {
    pub(in crate::cmd) fn execute(&self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_subsystem = core_fit.add_subsystem(self.type_id);
        if let Some(state) = self.state {
            core_subsystem.set_state(state);
        }
        self.effect_modes.apply(&mut core_subsystem);
        AddedItemIdsResp::from_core_subsystem(core_subsystem)
    }
}
