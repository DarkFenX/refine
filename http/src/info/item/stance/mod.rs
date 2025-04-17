use full::HStanceInfoFull;
use id::HStanceInfoId;
use partial::HStanceInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HStanceInfo {
    Id(HStanceInfoId),
    Partial(HStanceInfoPartial),
    Full(HStanceInfoFull),
}
impl HStanceInfo {
    pub(crate) fn mk_info(stance_implant: &mut rc::StanceMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(stance_implant.into()),
            HItemInfoMode::Partial => Self::Partial(stance_implant.into()),
            HItemInfoMode::Full => Self::Full(stance_implant.into()),
        }
    }
}
