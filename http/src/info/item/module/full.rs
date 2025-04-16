use crate::info::{HItemInfoMode, item::extended::HItemExtendedInfo};

use super::HModuleInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HModuleInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HModuleInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HModuleInfoFull {
    pub(super) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_module_info: &rc::ModuleInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        let partial_info = HModuleInfoPartial::mk_info(core_sol, core_module_info, item_mode);
        let extended_info = HItemExtendedInfo::from_item_id(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
