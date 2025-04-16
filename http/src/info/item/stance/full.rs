use crate::info::item::extended::HItemExtendedInfo;

use super::HStanceInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HStanceInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HStanceInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HStanceInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_stance_info: &rc::StanceInfo) -> Self {
        let partial_info = HStanceInfoPartial::from(core_stance_info);
        let extended_info = HItemExtendedInfo::from_item_id(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
