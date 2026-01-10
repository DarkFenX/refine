use serde::Serialize;

use super::{full::HShipInfoFull, id::HShipInfoId, partial::HShipInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HShipInfo {
    Id(HShipInfoId),
    Partial(HShipInfoPartial),
    Full(HShipInfoFull),
}
impl HShipInfo {
    pub(in crate::info::item) fn mk_info(core_ship: &mut rc::ShipMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_ship.into()),
            HItemInfoMode::Partial => Self::Partial(core_ship.into()),
            HItemInfoMode::Full => Self::Full(core_ship.into()),
        }
    }
}
