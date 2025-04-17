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
    pub(super) fn mk_info(core_module: &mut rc::ModuleMut, item_mode: HItemInfoMode) -> Self {
        Self {
            partial_info: HModuleInfoPartial::mk_info(core_module, item_mode),
            extended_info: core_module.into(),
        }
    }
}
