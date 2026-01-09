use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::shared::{HCoordinates, HMovement};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HShipInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    enabled: bool,
    coordinates: HCoordinates,
    movement: HMovement,
}
impl From<&mut rc::ShipMut<'_>> for HShipInfoPartial {
    fn from(core_ship: &mut rc::ShipMut) -> Self {
        Self {
            id: core_ship.get_item_id(),
            kind: "ship",
            type_id: core_ship.get_type_id().into_i32(),
            fit_id: core_ship.get_fit().get_fit_id(),
            enabled: core_ship.get_state(),
            coordinates: HCoordinates::from_core(core_ship.get_coordinates()),
            movement: HMovement::from_core(core_ship.get_movement()),
        }
    }
}
