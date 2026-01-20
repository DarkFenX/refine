use serde::Serialize;

use super::partial::HChargeInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HChargeInfoFull {
    #[serde(flatten)]
    partial_info: HChargeInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HChargeInfoFull {
    pub(super) fn from_core(core_charge: &mut rc::ChargeMut) -> Self {
        Self {
            partial_info: HChargeInfoPartial::from_core(core_charge),
            extended_info: HItemExtendedInfo::from_core(core_charge),
        }
    }
}
