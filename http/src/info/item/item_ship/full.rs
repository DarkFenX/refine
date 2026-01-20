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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HShipInfoFull {
    pub(super) fn from_core(core_ship: &mut rc::ShipMut) -> Self {
        Self {
            partial_info: HShipInfoPartial::from_core(core_ship),
            extended_info: HItemExtendedInfo::from_core(core_ship),
        }
    }
}
