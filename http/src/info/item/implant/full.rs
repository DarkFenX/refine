use crate::info::item::extended::HItemExtendedInfo;

use super::HImplantInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HImplantInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HImplantInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HImplantInfoFull {
    pub(super) fn from_item_id(core_sol: &mut rc::SolarSystem, implant_id: &rc::ItemId) -> Self {
        let partial_info = HImplantInfoPartial::from_item_id(core_sol, implant_id);
        let extended_info = HItemExtendedInfo::from_item_id(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
