use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct RigInfo {
    pub id: rc::ItemId,
    pub extended: Option<RigInfoExt>,
}

pub struct RigInfoExt {
    pub kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub state: bool,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RigInfo {
    pub(super) fn from_core(core_rig: &mut rc::RigMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_rig.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(RigInfoExt {
                    kind: rc::ItemKind::Rig,
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
