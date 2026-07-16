use rc::ItemCommon;

use super::shared::{ProjInfo, get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct ProjEffectInfo {
    pub id: rc::ItemId,
    pub extended: Option<ProjEffectInfoExt>,
}

pub struct ProjEffectInfoExt {
    kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub state: bool,
    // TODO: in serialization, rename to proj_item_ids
    pub projs: Vec<ProjInfo>,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ProjEffectInfo {
    pub(super) fn from_core(core_proj_effect: &mut rc::ProjEffectMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_proj_effect.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(ProjEffectInfoExt {
                    kind: rc::ItemKind::ProjEffect,
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
