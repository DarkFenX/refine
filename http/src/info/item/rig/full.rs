use crate::info::item::extended::HItemExtendedInfo;

use super::HRigInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HRigInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HRigInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HRigInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_rig_info: &rc::SolRigInfo) -> Self {
        let partial_info = HRigInfoPartial::from(core_rig_info);
        let extended_info = HItemExtendedInfo::mk_info(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
