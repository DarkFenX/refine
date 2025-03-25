use crate::info::item::extended::HItemExtendedInfo;

use super::HBoosterInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HBoosterInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HBoosterInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_booster_info: &rc::BoosterInfo) -> Self {
        let partial_info = HBoosterInfoPartial::mk_info(core_sol, core_booster_info);
        let extended_info = HItemExtendedInfo::mk_info(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
