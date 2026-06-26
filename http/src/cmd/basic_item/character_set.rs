use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HCmdResps, HItemIdsResp,
        shared::{HEffectModeMap, HFitIdBackref, get_primary_fit},
    },
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HCharacterSetCmdFCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HCharacterSetCmdICtx,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HCharacterSetCmdFCtxRIds {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    ictx_cmd: HCharacterSetCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HCharacterSetCmdICtx {
    type_id: i32,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HCharacterSetCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HCharacterSetCmdFCtxRIds, HExecError> {
        Ok(HCharacterSetCmdFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HCharacterSetCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fit_id)
    }
}

impl HCharacterSetCmdICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        let mut core_character = core_fit.set_character(core_type_id);
        if let Some(state) = self.state {
            core_character.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_character);
        }
        Ok(HItemIdsResp::from_core_character(core_character))
    }
}
