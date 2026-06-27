use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::shared::{HCmdResps, HCreatedItemIdsResp, HEffectModeMap, HFitIdBackref, get_primary_fit},
    shared::HServiceState,
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HServiceAddCmdFCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HServiceAddCmdICtx,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HServiceAddCmdFCtxRIds {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    ictx_cmd: HServiceAddCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HServiceAddCmdICtx {
    type_id: i32,
    state: HServiceState,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HServiceAddCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HServiceAddCmdFCtxRIds, HExecError> {
        Ok(HServiceAddCmdFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HServiceAddCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCreatedItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fit_id)
    }
}

impl HServiceAddCmdICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCreatedItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        let core_state = self.state.into_core();
        let mut core_service = core_fit.add_service(core_type_id, core_state);
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_service);
        }
        Ok(HCreatedItemIdsResp::from_core_service(core_service))
    }
}
