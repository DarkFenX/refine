use serde::Serialize;

use super::partial::HShipInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HShipInfoFull {
    #[serde(flatten)]
    partial_info: HShipInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}
impl From<&mut rc::ShipMut<'_>> for HShipInfoFull {
    fn from(core_ship: &mut rc::ShipMut) -> Self {
        Self {
            partial_info: core_ship.into(),
            extended_info: core_ship.into(),
        }
    }
}
