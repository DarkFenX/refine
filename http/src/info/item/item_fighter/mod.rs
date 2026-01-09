use full::HFighterInfoFull;
use id::HFighterInfoId;
use partial::HFighterInfoPartial;
use serde::Serialize;

use crate::info::HItemInfoMode;
mod ability;
mod full;
mod id;
mod partial;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HFighterInfo {
    Id(HFighterInfoId),
    Partial(HFighterInfoPartial),
    Full(HFighterInfoFull),
}
impl HFighterInfo {
    pub(in crate::info::item) fn mk_info(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HFighterInfoId::mk_info(core_fighter, item_mode)),
            HItemInfoMode::Partial => Self::Partial(HFighterInfoPartial::mk_info(core_fighter, item_mode)),
            HItemInfoMode::Full => Self::Full(HFighterInfoFull::mk_info(core_fighter, item_mode)),
        }
    }
}
