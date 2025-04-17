use crate::info::item::extended::HItemExtendedInfo;

use super::HChargeInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HChargeInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HChargeInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl From<&mut rc::ChargeMut<'_>> for HChargeInfoFull {
    fn from(core_charge: &mut rc::ChargeMut) -> Self {
        Self {
            partial_info: core_charge.into(),
            extended_info: core_charge.into(),
        }
    }
}
