use full::HFighterInfoFull;
use id::HFighterInfoId;
use partial::HFighterInfoPartial;

use crate::info::HItemInfoMode;

mod ability;
mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HFighterInfo {
    Id(HFighterInfoId),
    Partial(HFighterInfoPartial),
    Full(HFighterInfoFull),
}
impl HFighterInfo {
    pub(crate) fn mk_info(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HFighterInfoId::mk_info(core_fighter, item_mode)),
            HItemInfoMode::Partial => Self::Partial(HFighterInfoPartial::mk_info(core_fighter, item_mode)),
            HItemInfoMode::Full => Self::Full(HFighterInfoFull::mk_info(core_fighter, item_mode)),
        }
    }
}
