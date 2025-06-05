use full::HAutochargeInfoFull;
use id::HAutochargeInfoId;
use partial::HAutochargeInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HAutochargeInfo {
    Id(HAutochargeInfoId),
    Partial(HAutochargeInfoPartial),
    Full(HAutochargeInfoFull),
}
impl HAutochargeInfo {
    pub(crate) fn mk_info(core_autocharge: &mut rc::AutochargeMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_autocharge.into()),
            HItemInfoMode::Partial => Self::Partial(core_autocharge.into()),
            HItemInfoMode::Full => Self::Full(core_autocharge.into()),
        }
    }
}
