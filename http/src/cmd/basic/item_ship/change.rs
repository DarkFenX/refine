use rc::ItemCommon;
use serde::Deserialize;

use crate::{
    cmd::shared::{HChangedItemIdsResp, HCmdResps, HEffectModeMap, HFitIdBackref, HItemIdBackref, get_primary_fit},
    err::HExecError,
    shared::{HCoordinates, HMovement},
};

// Commands with full hybrid context
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum HShipChangeCmdFHybridCtxBIds {
    ViaFitId(HShipChangeCmdFFitCtxBIds),
    ViaItemId(HShipChangeCmdFItemCtxBIds),
}
pub(crate) enum HShipChangeCmdFHybridCtxRIds {
    ViaFitId(HShipChangeCmdFFitCtxRIds),
    ViaItemId(HShipChangeCmdFItemCtxRIds),
}

// Commands with full context via fit ID
#[derive(Deserialize)]
pub(crate) struct HShipChangeCmdFFitCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HShipChangeCmdICtx,
}
pub(crate) struct HShipChangeCmdFFitCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: HShipChangeCmdICtx,
}

// Commands with full context via item ID
#[derive(Deserialize)]
pub(crate) struct HShipChangeCmdFItemCtxBIds {
    item_id: HItemIdBackref,
    #[serde(flatten)]
    ictx_cmd: HShipChangeCmdICtx,
}
pub(crate) struct HShipChangeCmdFItemCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: HShipChangeCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HShipChangeCmdICtx {
    type_id: Option<i32>,
    state: Option<bool>,
    coordinates: Option<HCoordinates>,
    movement: Option<HMovement>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HShipChangeCmdFHybridCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HShipChangeCmdFHybridCtxRIds, HExecError> {
        Ok(match self {
            Self::ViaFitId(cmd) => HShipChangeCmdFHybridCtxRIds::ViaFitId(cmd.render(resps)?),
            Self::ViaItemId(cmd) => HShipChangeCmdFHybridCtxRIds::ViaItemId(cmd.render(resps)?),
        })
    }
}
impl HShipChangeCmdFFitCtxBIds {
    fn render(self, resps: &HCmdResps) -> Result<HShipChangeCmdFFitCtxRIds, HExecError> {
        Ok(HShipChangeCmdFFitCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}
impl HShipChangeCmdFItemCtxBIds {
    fn render(self, resps: &HCmdResps) -> Result<HShipChangeCmdFItemCtxRIds, HExecError> {
        Ok(HShipChangeCmdFItemCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HShipChangeCmdFHybridCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        match self {
            Self::ViaFitId(cmd) => cmd.execute(core_sol),
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
        }
    }
}
impl HShipChangeCmdFFitCtxRIds {
    fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        self.ictx_cmd.execute_via_fit_id(core_sol, &self.fit_id)
    }
}
impl HShipChangeCmdFItemCtxRIds {
    fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        self.ictx_cmd.execute_via_item_id(core_sol, &self.item_id)
    }
}

impl HShipChangeCmdICtx {
    pub(in crate::cmd) fn execute_via_fit_id(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HChangedItemIdsResp, HExecError> {
        let core_fit = get_primary_fit(core_sol, fit_id)?;
        let ship_item_id = match core_fit.get_ship() {
            Some(core_ship) => core_ship.get_item_id(),
            None => return Err(HExecError::FitShipNotFound(*fit_id)),
        };
        self.execute_via_item_id(core_sol, &ship_item_id)
    }
    pub(in crate::cmd) fn execute_via_item_id(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HChangedItemIdsResp, HExecError> {
        let mut core_ship = core_sol.get_ship_mut(item_id).map_err(|error| match error {
            rc::err::GetShipError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetShipError::ItemIsNotShip(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_ship.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_ship.set_state(state);
        }
        if let Some(coordinates) = self.coordinates {
            core_ship.set_coordinates(coordinates.into_core());
        }
        if let Some(movement) = self.movement {
            core_ship.set_movement(movement.into_core());
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_ship);
        }
        Ok(HChangedItemIdsResp::default())
    }
}
