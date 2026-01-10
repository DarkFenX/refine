use serde::Serialize;

use super::{full::HAutochargeInfoFull, id::HAutochargeInfoId, partial::HAutochargeInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HAutochargeInfo {
    Id(HAutochargeInfoId),
    Partial(HAutochargeInfoPartial),
    Full(HAutochargeInfoFull),
}
impl HAutochargeInfo {
    pub(in crate::info::item) fn mk_info(core_autocharge: &mut rc::AutochargeMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_autocharge.into()),
            HItemInfoMode::Partial => Self::Partial(core_autocharge.into()),
            HItemInfoMode::Full => Self::Full(core_autocharge.into()),
        }
    }
}
