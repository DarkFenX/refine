use full::HImplantInfoFull;
use id::HImplantInfoId;
use partial::HImplantInfoPartial;
use serde::Serialize;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HImplantInfo {
    Id(HImplantInfoId),
    Partial(HImplantInfoPartial),
    Full(HImplantInfoFull),
}
impl HImplantInfo {
    pub(in crate::info::item) fn mk_info(core_implant: &mut rc::ImplantMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_implant.into()),
            HItemInfoMode::Partial => Self::Partial(core_implant.into()),
            HItemInfoMode::Full => Self::Full(core_implant.into()),
        }
    }
}
