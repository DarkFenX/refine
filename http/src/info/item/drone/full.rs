use crate::info::HItemExtendedInfo;

use super::HDroneInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HDroneInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HDroneInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HDroneInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_drone_info: &rc::SolDroneInfo) -> Self {
        let partial_info = HDroneInfoPartial::from(core_drone_info);
        let extended_info = HItemExtendedInfo::mk_info(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
