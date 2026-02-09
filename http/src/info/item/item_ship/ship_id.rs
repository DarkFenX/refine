use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HShipInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HShipInfoId {
    pub(super) fn from_core(core_ship: &mut rc::ShipMut) -> Self {
        Self {
            id: core_ship.get_item_id(),
        }
    }
}
