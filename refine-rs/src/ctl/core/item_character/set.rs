use crate::{
    AddedItemIdsResp, CtlCmdResps, FitId, FitIdBr, ItemTypeId, ctl::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdCharacterSetFCtxBIds {
    pub(in crate::ctl) fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdCharacterSetICtx,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdCharacterSetFCtxRIds {
    pub(in crate::ctl) fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdCharacterSetICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdCharacterSetICtx {
    pub(in crate::ctl) type_id: ItemTypeId,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdCharacterSetFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdCharacterSetFCtxRIds, BackrefRenderError> {
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
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitSetCharacterError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitSetCharacterError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
}

impl ICmdCharacterSetICtx {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_character = core_fit.set_character(self.type_id);
        if let Some(state) = self.state {
            core_character.set_state(state);
        }
        self.effect_modes.apply(&mut core_character);
        AddedItemIdsResp::from_core_character(core_character)
    }
}
