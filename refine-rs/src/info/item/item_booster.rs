use rc::{ItemCommon, Lender};

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, AttrVals, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification, SideEffectInfo,
    SlotIndex,
};

pub struct BoosterInfo {
    pub id: ItemId,
    pub extended: Option<BoosterInfoExt>,
}

pub struct BoosterInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub slot: Option<SlotIndex>,
    pub state: bool,
    pub side_effects: Vec<(EffectId, SideEffectInfo)>,
    pub attrs: Vec<(AttrId, AttrVals)>,
    pub effects: Vec<(EffectId, EffectInfo)>,
    pub mods: Vec<(AttrId, Vec<Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BoosterInfo {
    pub(in crate::info) fn from_core(core_booster: &mut rc::BoosterMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_booster.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(BoosterInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Booster,
                    type_id: core_booster.get_type_id(),
                    fit_id: core_booster.get_fit().get_fit_id(),
                    slot: core_booster.get_slot(),
                    state: core_booster.get_state(),
                    side_effects: core_booster
                        .iter_side_effects_mut()
                        .map_into_iter(|core_side_effect| {
                            (
                                core_side_effect.get_effect_id(),
                                SideEffectInfo::from_core(core_side_effect),
                            )
                        })
                        .collect(),
                    attrs: get_attrs(core_booster, item_mode),
                    effects: get_effects(core_booster, item_mode),
                    mods: get_mods(core_booster, item_mode),
                }),
            },
        }
    }
}
