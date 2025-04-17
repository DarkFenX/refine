use full::HImplantInfoFull;
use id::HImplantInfoId;
use partial::HImplantInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HImplantInfo {
    Id(HImplantInfoId),
    Partial(HImplantInfoPartial),
    Full(HImplantInfoFull),
}
impl HImplantInfo {
    pub(crate) fn mk_info(core_implant: &mut rc::ImplantMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_implant.into()),
            HItemInfoMode::Partial => Self::Partial(core_implant.into()),
            HItemInfoMode::Full => Self::Full(core_implant.into()),
        }
    }
}
