use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{AttrId, AttrVals, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification};

pub struct RigInfo {
    pub id: ItemId,
    pub extended: Option<RigInfoExt>,
}

pub struct RigInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: bool,
    pub attrs: Vec<(AttrId, AttrVals)>,
    pub effects: Vec<(EffectId, EffectInfo)>,
    pub mods: Vec<(AttrId, Vec<Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RigInfo {
    pub(in crate::info) fn from_core(core_rig: &mut rc::RigMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_rig.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(RigInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Rig,
                    type_id: core_rig.get_type_id(),
                    fit_id: core_rig.get_fit().get_fit_id(),
                    state: core_rig.get_state(),
                    attrs: get_attrs(core_rig, item_mode),
                    effects: get_effects(core_rig, item_mode),
                    mods: get_mods(core_rig, item_mode),
                }),
            },
        }
    }
}
