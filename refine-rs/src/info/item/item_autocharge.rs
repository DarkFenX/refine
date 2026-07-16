use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{AttrId, AttrVals, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification};

pub struct AutochargeInfo {
    pub id: ItemId,
    pub extended: Option<AutochargeInfoExt>,
}

pub struct AutochargeInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub cont_item_id: ItemId,
    pub cont_effect_id: EffectId,
    pub state: bool,
    pub attrs: Vec<(AttrId, AttrVals)>,
    pub effects: Vec<(EffectId, EffectInfo)>,
    pub mods: Vec<(AttrId, Vec<Modification>)>,
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
                    kind: ItemKind::Autocharge,
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
