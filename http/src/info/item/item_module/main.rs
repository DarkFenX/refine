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
impl HModuleInfo {
    pub(in crate::info::item) fn mk_info(core_module: &mut rc::ModuleMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HModuleInfoId::mk_info(core_module, item_mode)),
            HItemInfoMode::Partial => Self::Partial(HModuleInfoPartial::mk_info(core_module, item_mode)),
            HItemInfoMode::Full => Self::Full(HModuleInfoFull::mk_info(core_module, item_mode)),
        }
    }
}
