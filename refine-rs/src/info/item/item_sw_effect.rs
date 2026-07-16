use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct SwEffectInfo {
    pub id: rc::ItemId,
    pub extended: Option<SwEffectInfoExt>,
}

pub struct SwEffectInfoExt {
    #[cfg(feature = "serde")]
    kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub state: bool,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SwEffectInfo {
    pub(super) fn from_core(core_sw_effect: &mut rc::SwEffectMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_sw_effect.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(SwEffectInfoExt {
                    #[cfg(feature = "serde")]
                    kind: rc::ItemKind::SwEffect,
                    type_id: core_sw_effect.get_type_id(),
                    state: core_sw_effect.get_state(),
                    attrs: get_attrs(core_sw_effect, item_mode),
                    effects: get_effects(core_sw_effect, item_mode),
                    mods: get_mods(core_sw_effect, item_mode),
                }),
            },
        }
    }
}
