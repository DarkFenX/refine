use rc::ItemCommon;
use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp, old_change_item,
        shared::{HEffectModeMap, get_primary_fit},
    },
    shared::{HCoordinates, HMovement},
    util::HExecError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Setting
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
pub(crate) struct HSetShipCmd {
    type_id: i32,
    state: Option<bool>,
    coordinates: Option<HCoordinates>,
    movement: Option<HMovement>,
    effect_modes: Option<HEffectModeMap>,
}
impl HSetShipCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        let mut core_ship = core_fit.set_ship(
            core_type_id,
            self.coordinates.map(|v| v.into_core()),
            self.movement.map(|v| v.into_core()),
        );
        if let Some(state) = self.state {
            core_ship.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_ship);
        }
        Ok(HItemIdsResp::from_core_ship(core_ship))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Changing
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeShipCmd {
    ViaItemId(HChangeShipViaItemIdCmd),
    ViaFitId(HChangeShipViaFitIdCmd),
}
impl HChangeShipCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
            Self::ViaFitId(cmd) => cmd.execute(core_sol, fit_id),
        }
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeShipViaItemIdCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: old_change_item::HChangeShipCmd,
}
impl HChangeShipViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}

#[derive(Deserialize)]
pub(crate) struct HChangeShipViaFitIdCmd {
    #[serde(flatten)]
    item_cmd: old_change_item::HChangeShipCmd,
}
impl HChangeShipViaFitIdCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let core_fit = get_primary_fit(core_sol, fit_id)?;
        let ship_item_id = match core_fit.get_ship() {
            Some(core_ship) => core_ship.get_item_id(),
            None => return Err(HExecError::FitShipNotFound(*fit_id)),
        };
        self.item_cmd.execute(core_sol, &ship_item_id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Unsetting
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
pub(crate) struct HUnsetShipCmd;
impl HUnsetShipCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<(), HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        if let Some(core_ship) = core_fit.get_ship_mut() {
            core_ship.remove();
        }
        Ok(())
    }
}
