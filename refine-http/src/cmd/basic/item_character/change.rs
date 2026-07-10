use rc::ItemCommon;
use serde::Deserialize;

use crate::{
    cmd::shared::{HChangedItemIdsResp, HCmdResps, HEffectModeMap, HFitIdBackref, HItemIdBackref, get_primary_fit},
    err::HExecError,
};

// Commands with full hybrid context
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum HCharacterChangeCmdFHybridCtxBIds {
    ViaFitId(HCharacterChangeCmdFFitCtxBIds),
    ViaItemId(HCharacterChangeCmdFItemCtxBIds),
}
pub(crate) enum HCharacterChangeCmdFHybridCtxRIds {
    ViaFitId(HCharacterChangeCmdFFitCtxRIds),
    ViaItemId(HCharacterChangeCmdFItemCtxRIds),
}

// Commands with full context via fit ID
#[derive(Deserialize)]
pub(crate) struct HCharacterChangeCmdFFitCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HCharacterChangeCmdICtx,
}
pub(crate) struct HCharacterChangeCmdFFitCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: HCharacterChangeCmdICtx,
}

// Commands with full context via item ID
#[derive(Deserialize)]
pub(crate) struct HCharacterChangeCmdFItemCtxBIds {
    item_id: HItemIdBackref,
    #[serde(flatten)]
    ictx_cmd: HCharacterChangeCmdICtx,
}
pub(crate) struct HCharacterChangeCmdFItemCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: HCharacterChangeCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HCharacterChangeCmdICtx {
    type_id: Option<i32>,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HCharacterChangeCmdFHybridCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HCharacterChangeCmdFHybridCtxRIds, HExecError> {
        Ok(match self {
            Self::ViaFitId(cmd) => HCharacterChangeCmdFHybridCtxRIds::ViaFitId(cmd.render(resps)?),
            Self::ViaItemId(cmd) => HCharacterChangeCmdFHybridCtxRIds::ViaItemId(cmd.render(resps)?),
        })
    }
}
impl HCharacterChangeCmdFFitCtxBIds {
    fn render(self, resps: &HCmdResps) -> Result<HCharacterChangeCmdFFitCtxRIds, HExecError> {
        Ok(HCharacterChangeCmdFFitCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}
impl HCharacterChangeCmdFItemCtxBIds {
    fn render(self, resps: &HCmdResps) -> Result<HCharacterChangeCmdFItemCtxRIds, HExecError> {
        Ok(HCharacterChangeCmdFItemCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HCharacterChangeCmdFHybridCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        match self {
            Self::ViaFitId(cmd) => cmd.execute(core_sol),
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
        }
    }
}
impl HCharacterChangeCmdFFitCtxRIds {
    fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        self.ictx_cmd.execute_via_fit_id(core_sol, &self.fit_id)
    }
}
impl HCharacterChangeCmdFItemCtxRIds {
    fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        self.ictx_cmd.execute_via_item_id(core_sol, &self.item_id)
    }
}

impl HCharacterChangeCmdICtx {
    pub(in crate::cmd) fn execute_via_fit_id(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HChangedItemIdsResp, HExecError> {
        let core_fit = get_primary_fit(core_sol, fit_id)?;
        let character_item_id = match core_fit.get_character() {
            Some(core_character) => core_character.get_item_id(),
            None => return Err(HExecError::FitCharacterNotFound(*fit_id)),
        };
        self.execute_via_item_id(core_sol, &character_item_id)
    }
    pub(in crate::cmd) fn execute_via_item_id(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HChangedItemIdsResp, HExecError> {
        let mut core_character = core_sol.get_character_mut(item_id).map_err(|error| match error {
            rc::err::GetCharacterError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetCharacterError::ItemIsNotCharacter(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_character.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_character.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_character);
        }
        Ok(HChangedItemIdsResp::default())
    }
}
