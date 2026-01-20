use serde::Serialize;

use super::partial::HModuleInfoPartial;
use crate::info::{HItemInfoMode, item::extended::HItemExtendedInfo};

#[derive(Serialize)]
pub(crate) struct HModuleInfoFull {
    #[serde(flatten)]
    partial_info: HModuleInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HModuleInfoFull {
    pub(super) fn from_core(core_module: &mut rc::ModuleMut, item_mode: HItemInfoMode) -> Self {
        Self {
            partial_info: HModuleInfoPartial::from_core(core_module, item_mode),
            extended_info: HItemExtendedInfo::from_core(core_module),
        }
    }
}
