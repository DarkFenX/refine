use rc::ItemCommon;
use serde::Deserialize;

use crate::{
    cmd::shared::{HChangedItemIdsResp, HCmdResps, HEffectModeMap, HFitIdBackref, HItemIdBackref, get_primary_fit},
    err::HExecError,
};

// Commands with full hybrid context
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum HStanceChangeCmdFHybridCtxBIds {
    ViaFitId(HStanceChangeCmdFFitCtxBIds),
    ViaItemId(HStanceChangeCmdFItemCtxBIds),
}
pub(crate) enum HStanceChangeCmdFHybridCtxRIds {
    ViaFitId(HStanceChangeCmdFFitCtxRIds),
    ViaItemId(HStanceChangeCmdFItemCtxRIds),
}

// Commands with full context via fit ID
#[derive(Deserialize)]
pub(crate) struct HStanceChangeCmdFFitCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HStanceChangeCmdICtx,
}
pub(crate) struct HStanceChangeCmdFFitCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: HStanceChangeCmdICtx,
}

// Commands with full context via item ID
#[derive(Deserialize)]
pub(crate) struct HStanceChangeCmdFItemCtxBIds {
    item_id: HItemIdBackref,
    #[serde(flatten)]
    ictx_cmd: HStanceChangeCmdICtx,
}
pub(crate) struct HStanceChangeCmdFItemCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: HStanceChangeCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HStanceChangeCmdICtx {
    type_id: Option<i32>,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStanceChangeCmdFHybridCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HStanceChangeCmdFHybridCtxRIds, HExecError> {
        Ok(match self {
            Self::ViaFitId(cmd) => HStanceChangeCmdFHybridCtxRIds::ViaFitId(cmd.render(resps)?),
            Self::ViaItemId(cmd) => HStanceChangeCmdFHybridCtxRIds::ViaItemId(cmd.render(resps)?),
        })
    }
}
impl HStanceChangeCmdFFitCtxBIds {
    fn render(self, resps: &HCmdResps) -> Result<HStanceChangeCmdFFitCtxRIds, HExecError> {
        Ok(HStanceChangeCmdFFitCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}
impl HStanceChangeCmdFItemCtxBIds {
    fn render(self, resps: &HCmdResps) -> Result<HStanceChangeCmdFItemCtxRIds, HExecError> {
        Ok(HStanceChangeCmdFItemCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStanceChangeCmdFHybridCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        match self {
            Self::ViaFitId(cmd) => cmd.execute(core_sol),
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
        }
    }
}
impl HStanceChangeCmdFFitCtxRIds {
    fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        self.ictx_cmd.execute_via_fit_id(core_sol, &self.fit_id)
    }
}
impl HStanceChangeCmdFItemCtxRIds {
    fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        self.ictx_cmd.execute_via_item_id(core_sol, &self.item_id)
    }
}

impl HStanceChangeCmdICtx {
    pub(in crate::cmd) fn execute_via_fit_id(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HChangedItemIdsResp, HExecError> {
        let core_fit = get_primary_fit(core_sol, fit_id)?;
        let stance_item_id = match core_fit.get_stance() {
            Some(core_stance) => core_stance.get_item_id(),
            None => return Err(HExecError::FitStanceNotFound(*fit_id)),
        };
        self.execute_via_item_id(core_sol, &stance_item_id)
    }
    pub(in crate::cmd) fn execute_via_item_id(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HChangedItemIdsResp, HExecError> {
        let mut core_stance = core_sol.get_stance_mut(item_id).map_err(|error| match error {
            rc::err::GetStanceError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetStanceError::ItemIsNotStance(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_stance.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_stance.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_stance);
        }
        Ok(HChangedItemIdsResp::default())
    }
}
