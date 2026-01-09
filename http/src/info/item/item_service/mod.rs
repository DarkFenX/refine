use full::HServiceInfoFull;
use id::HServiceInfoId;
use partial::HServiceInfoPartial;
use serde::Serialize;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HServiceInfo {
    Id(HServiceInfoId),
    Partial(HServiceInfoPartial),
    Full(HServiceInfoFull),
}
impl HServiceInfo {
    pub(in crate::info::item) fn mk_info(core_service: &mut rc::ServiceMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_service.into()),
            HItemInfoMode::Partial => Self::Partial(core_service.into()),
            HItemInfoMode::Full => Self::Full(core_service.into()),
        }
    }
}
