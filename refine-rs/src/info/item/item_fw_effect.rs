use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct FwEffectInfo {
    pub id: rc::ItemId,
    pub extended: Option<FwEffectInfoExt>,
}

pub struct FwEffectInfoExt {
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
impl FwEffectInfo {
    pub(super) fn from_core(core_fw_effect: &mut rc::FwEffectMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_fw_effect.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(FwEffectInfoExt {
                    kind: rc::ItemKind::FwEffect,
                    type_id: core_fw_effect.get_type_id(),
                    fit_id: core_fw_effect.get_fit().get_fit_id(),
                    state: core_fw_effect.get_state(),
                    attrs: get_attrs(core_fw_effect, item_mode),
                    effects: get_effects(core_fw_effect, item_mode),
                    mods: get_mods(core_fw_effect, item_mode),
                }),
            },
        }
    }
}
