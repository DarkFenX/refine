use crate::info::item::extended::HItemExtendedInfo;

use super::HSubsystemInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HSubsystemInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HSubsystemInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HSubsystemInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_subsystem_info: &rc::SolSubsystemInfo) -> Self {
        let partial_info = HSubsystemInfoPartial::from(core_subsystem_info);
        let extended_info = HItemExtendedInfo::mk_info(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
