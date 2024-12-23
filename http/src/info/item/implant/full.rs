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
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_implant_info: &rc::SolImplantInfo) -> Self {
        let partial_info = HImplantInfoPartial::from(core_implant_info);
        let extended_info = HItemExtendedInfo::mk_info(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
