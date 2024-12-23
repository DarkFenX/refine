use crate::info::HItemExtendedInfo;

use super::HAutochargeInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HAutochargeInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HAutochargeInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HAutochargeInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_autocharge_info: &rc::SolAutochargeInfo) -> Self {
        let partial_info = HAutochargeInfoPartial::from(core_autocharge_info);
        let extended_info = HItemExtendedInfo::mk_info(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
