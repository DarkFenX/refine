use serde::Serialize;

use super::{ship_full::HShipInfoFull, ship_id::HShipInfoId, ship_partial::HShipInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HShipInfo {
    Id(HShipInfoId),
    Partial(HShipInfoPartial),
    Full(HShipInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HShipInfo {
    pub(in crate::info::item) fn from_core(core_ship: &mut rc::ShipMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HShipInfoId::from_core(core_ship)),
            HItemInfoMode::Partial => Self::Partial(HShipInfoPartial::from_core(core_ship)),
            HItemInfoMode::Full => Self::Full(HShipInfoFull::from_core(core_ship)),
        }
    }
}
