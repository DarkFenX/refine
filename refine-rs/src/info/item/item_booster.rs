use rc::{ItemCommon, Lender};

use super::shared::{SideEffectInfo, get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct BoosterInfo {
    pub id: rc::ItemId,
    pub extended: Option<BoosterInfoExt>,
}

pub struct BoosterInfoExt {
    pub kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub slot: Option<rc::SlotIndex>,
    pub state: bool,
    pub side_effects: Vec<(rc::EffectId, SideEffectInfo)>,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
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
                    kind: rc::ItemKind::Booster,
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
