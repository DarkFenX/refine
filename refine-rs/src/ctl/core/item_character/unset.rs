use crate::{CmdResps, FitId, FitIdBr, err::BrResolveError};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct CharacterUnsetCmd;

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct CharacterUnsetCmdCtxFit {
    fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: CharacterUnsetCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct CharacterUnsetCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: CharacterUnsetCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CharacterUnsetCmd {
    pub fn new() -> Self {
        Self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CharacterUnsetCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> CharacterUnsetCmdCtxFit {
        CharacterUnsetCmdCtxFit { fit_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> CharacterUnsetCmdCtxFitBr {
        CharacterUnsetCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CharacterUnsetCmdCtxFitBr {
    pub(in crate::ctl) fn render(self, resps: &CmdResps) -> Result<CharacterUnsetCmdCtxFit, BrResolveError> {
        Ok(CharacterUnsetCmdCtxFit {
            fit_id: resps.render_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CharacterUnsetCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) {
        if let Some(core_character) = core_fit.get_character_mut() {
            core_character.remove();
        }
    }
}

impl CharacterUnsetCmdCtxFit {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), FitGetCharacterUnsetError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        self.core.execute(&mut core_fit);
        Ok(())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetCharacterUnsetError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
}
