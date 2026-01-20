use serde::Serialize;

use super::{full::HModuleInfoFull, id::HModuleInfoId, partial::HModuleInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HModuleInfo {
    Id(HModuleInfoId),
    Partial(HModuleInfoPartial),
    Full(HModuleInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HModuleInfo {
    pub(in crate::info::item) fn from_core(core_module: &mut rc::ModuleMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HModuleInfoId::from_core(core_module, item_mode)),
            HItemInfoMode::Partial => Self::Partial(HModuleInfoPartial::from_core(core_module, item_mode)),
            HItemInfoMode::Full => Self::Full(HModuleInfoFull::from_core(core_module, item_mode)),
        }
    }
}
