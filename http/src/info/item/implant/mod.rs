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
    pub(crate) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_implant_id: &rc::ItemId,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HImplantInfoId::from_item_id(core_implant_id)),
            HItemInfoMode::Partial => Self::Partial(HImplantInfoPartial::from_item_id(core_sol, core_implant_id)),
            HItemInfoMode::Full => Self::Full(HImplantInfoFull::from_item_id(core_sol, core_implant_id)),
        }
    }
}
