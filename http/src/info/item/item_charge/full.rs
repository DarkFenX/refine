use serde::Serialize;

use super::HChargeInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HChargeInfoFull {
    #[serde(flatten)]
    partial_info: HChargeInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}
impl From<&mut rc::ChargeMut<'_>> for HChargeInfoFull {
    fn from(core_charge: &mut rc::ChargeMut) -> Self {
        Self {
            partial_info: core_charge.into(),
            extended_info: core_charge.into(),
        }
    }
}
