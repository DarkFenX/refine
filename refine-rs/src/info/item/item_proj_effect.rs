use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{AttrId, AttrVals, EffectId, EffectInfo, ItemId, ItemInfoMode, ItemTypeId, Modification, ProjInfo};

pub struct ProjEffectInfo {
    pub id: ItemId,
    pub extended: Option<ProjEffectInfoExt>,
}

pub struct ProjEffectInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub state: bool,
    // TODO: in serialization, rename to proj_item_ids
    pub projs: Vec<ProjInfo>,
    pub attrs: Vec<(AttrId, AttrVals)>,
    pub effects: Vec<(EffectId, EffectInfo)>,
    pub mods: Vec<(AttrId, Vec<Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ProjEffectInfo {
    pub(in crate::info) fn from_core(core_proj_effect: &mut rc::ProjEffectMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_proj_effect.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(ProjEffectInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::ProjEffect,
                    type_id: core_proj_effect.get_type_id(),
                    state: core_proj_effect.get_state(),
                    projs: core_proj_effect.iter_projs().map(ProjInfo::from_core).collect(),
                    attrs: get_attrs(core_proj_effect, item_mode),
                    effects: get_effects(core_proj_effect, item_mode),
                    mods: get_mods(core_proj_effect, item_mode),
                }),
            },
        }
    }
}
