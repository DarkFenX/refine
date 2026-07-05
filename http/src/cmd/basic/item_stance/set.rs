use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::shared::{HCmdResps, HCreatedItemIdsResp, HEffectModeMap, HFitIdBackref, get_primary_fit},
    err::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HStanceSetCmdFCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HStanceSetCmdICtx,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HStanceSetCmdFCtxRIds {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    ictx_cmd: HStanceSetCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HStanceSetCmdICtx {
    type_id: i32,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStanceSetCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HStanceSetCmdFCtxRIds, HExecError> {
        Ok(HStanceSetCmdFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStanceSetCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCreatedItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fit_id)
    }
}

impl HStanceSetCmdICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCreatedItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        let mut core_stance = core_fit.set_stance(core_type_id);
        if let Some(state) = self.state {
            core_stance.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_stance);
        }
        Ok(HCreatedItemIdsResp::from_core_stance(core_stance))
    }
}
