use crate::{AddedItemIdsResp, CmdResps, FitIdBackref, cmd::shared::EffectModes, err::BackrefRenderError};

// Commands with full context
pub(in crate::cmd) struct ICmdCharacterSetFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdCharacterSetICtx,
}
pub(crate) struct ICmdCharacterSetFCtxRIds {
    pub(in crate::cmd) fit_id: rc::FitId,
    pub(in crate::cmd) ictx_cmd: ICmdCharacterSetICtx,
}

// Commands with incomplete context
pub(crate) struct ICmdCharacterSetICtx {
    pub(in crate::cmd) type_id: rc::ItemTypeId,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdCharacterSetFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdCharacterSetFCtxRIds, BackrefRenderError> {
        Ok(ICmdCharacterSetFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdCharacterSetFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitSetCharacterError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitSetCharacterError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
}

impl ICmdCharacterSetICtx {
    pub(in crate::cmd) fn execute(self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_character = core_fit.set_character(self.type_id);
        if let Some(state) = self.state {
            core_character.set_state(state);
        }
        self.effect_modes.apply(&mut core_character);
        AddedItemIdsResp::from_core_character(core_character)
    }
}
