use crate::info::item::extended::HItemExtendedInfo;

use super::HShipInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HShipInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HShipInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HShipInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_ship_info: &rc::ShipInfo) -> Self {
        let partial_info = HShipInfoPartial::from(core_ship_info);
        let extended_info = HItemExtendedInfo::from_item_id(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
