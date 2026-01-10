use serde::Serialize;

use super::{full::HChargeInfoFull, id::HChargeInfoId, partial::HChargeInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HChargeInfo {
    Id(HChargeInfoId),
    Partial(HChargeInfoPartial),
    Full(HChargeInfoFull),
}
impl HChargeInfo {
    pub(in crate::info::item) fn mk_info(core_charge: &mut rc::ChargeMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_charge.into()),
            HItemInfoMode::Partial => Self::Partial(core_charge.into()),
            HItemInfoMode::Full => Self::Full(core_charge.into()),
        }
    }
}
