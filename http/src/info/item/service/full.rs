use crate::info::item::extended::HItemExtendedInfo;

use super::HServiceInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HServiceInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HServiceInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HServiceInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_service_info: &rc::ServiceInfo) -> Self {
        let partial_info = HServiceInfoPartial::from(core_service_info);
        let extended_info = HItemExtendedInfo::from_item_id(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
