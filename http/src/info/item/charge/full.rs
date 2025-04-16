use crate::info::item::extended::HItemExtendedInfo;

use super::HChargeInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HChargeInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HChargeInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HChargeInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_charge_info: &rc::ChargeInfo) -> Self {
        let partial_info = HChargeInfoPartial::from(core_charge_info);
        let extended_info = HItemExtendedInfo::from_item_id(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
