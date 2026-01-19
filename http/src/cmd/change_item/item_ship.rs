use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    shared::{HCoordinates, HMovement},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HChangeShipCmd {
    #[serde(default)]
    type_id: Option<i32>,
    #[serde(default)]
    state: Option<bool>,
    #[serde(default)]
    coordinates: Option<HCoordinates>,
    #[serde(default)]
    movement: Option<HMovement>,
    #[serde(default)]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeShipCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
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
        apply_effect_modes(&mut core_ship, &self.effect_modes);
        Ok(HItemIdsResp::from_core_ship(core_ship))
    }
}
