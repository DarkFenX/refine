use full::HModuleInfoFull;
use id::HModuleInfoId;
use partial::HModuleInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HModuleInfo {
    Id(HModuleInfoId),
    Partial(HModuleInfoPartial),
    Full(HModuleInfoFull),
}
impl HModuleInfo {
    pub(crate) fn mk_info(core_module: &mut rc::ModuleMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HModuleInfoId::mk_info(core_module, item_mode)),
            HItemInfoMode::Partial => Self::Partial(HModuleInfoPartial::mk_info(core_module, item_mode)),
            HItemInfoMode::Full => Self::Full(HModuleInfoFull::mk_info(core_module, item_mode)),
        }
    }
}
