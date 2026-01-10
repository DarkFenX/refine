use serde::Serialize;

use super::{full::HBoosterInfoFull, id::HBoosterInfoId, partial::HBoosterInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HBoosterInfo {
    Id(HBoosterInfoId),
    Partial(HBoosterInfoPartial),
    Full(HBoosterInfoFull),
}
impl HBoosterInfo {
    pub(in crate::info::item) fn mk_info(core_booster: &mut rc::BoosterMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_booster.into()),
            HItemInfoMode::Partial => Self::Partial(core_booster.into()),
            HItemInfoMode::Full => Self::Full(core_booster.into()),
        }
    }
}
