use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HCmdResps,
        shared::{HFitIdBackref, get_primary_fit},
    },
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HCharacterUnsetCmdFCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HCharacterUnsetCmdICtx,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HCharacterUnsetCmdFCtxRIds {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    ictx_cmd: HCharacterUnsetCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HCharacterUnsetCmdICtx;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HCharacterUnsetCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HCharacterUnsetCmdFCtxRIds, HExecError> {
        Ok(HCharacterUnsetCmdFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HCharacterUnsetCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fit_id)
    }
}

impl HCharacterUnsetCmdICtx {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<(), HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        if let Some(core_character) = core_fit.get_character_mut() {
            core_character.remove();
        }
        Ok(())
    }
}
