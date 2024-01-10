use full::HStructureInfoFull;
use id::HStructureInfoId;
use partial::HStructureInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HStructureInfo {
    Id(HStructureInfoId),
    Partial(HStructureInfoPartial),
    Full(HStructureInfoFull),
}
impl HStructureInfo {
    pub(crate) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_structure_info: &rc::SsStructureInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_structure_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_structure_info.into()),
            HItemInfoMode::Full => Self::Full(HStructureInfoFull::mk_info(core_ss, core_structure_info)),
        }
    }
}
