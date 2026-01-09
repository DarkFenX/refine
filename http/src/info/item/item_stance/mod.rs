use full::HStanceInfoFull;
use id::HStanceInfoId;
use partial::HStanceInfoPartial;
use serde::Serialize;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HStanceInfo {
    Id(HStanceInfoId),
    Partial(HStanceInfoPartial),
    Full(HStanceInfoFull),
}
impl HStanceInfo {
    pub(in crate::info::item) fn mk_info(stance_implant: &mut rc::StanceMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(stance_implant.into()),
            HItemInfoMode::Partial => Self::Partial(stance_implant.into()),
            HItemInfoMode::Full => Self::Full(stance_implant.into()),
        }
    }
}
