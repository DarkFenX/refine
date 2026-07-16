use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct AutochargeInfo {
    pub id: rc::ItemId,
    pub extended: Option<AutochargeInfoExt>,
}

pub struct AutochargeInfoExt {
    #[cfg(feature = "serde")]
    kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub cont_item_id: rc::ItemId,
    pub cont_effect_id: rc::EffectId,
    pub state: bool,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AutochargeInfo {
    pub(super) fn from_core(core_autocharge: &mut rc::AutochargeMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_autocharge.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(AutochargeInfoExt {
                    #[cfg(feature = "serde")]
                    kind: rc::ItemKind::Autocharge,
                    type_id: core_autocharge.get_type_id(),
                    fit_id: core_autocharge.get_fit().get_fit_id(),
                    cont_item_id: core_autocharge.get_cont_item().get_item_id(),
                    cont_effect_id: core_autocharge.get_cont_effect_id(),
                    state: core_autocharge.get_state(),
                    attrs: get_attrs(core_autocharge, item_mode),
                    effects: get_effects(core_autocharge, item_mode),
                    mods: get_mods(core_autocharge, item_mode),
                }),
            },
        }
    }
}
