use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{AttrId, AttrVals, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification, SlotIndex};

pub struct SubsystemInfo {
    pub id: ItemId,
    pub extended: Option<SubsystemInfoExt>,
}

pub struct SubsystemInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub slot: Option<SlotIndex>,
    pub state: bool,
    pub attrs: Vec<(AttrId, AttrVals)>,
    pub effects: Vec<(EffectId, EffectInfo)>,
    pub mods: Vec<(AttrId, Vec<Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SubsystemInfo {
    pub(in crate::info) fn from_core(core_subsystem: &mut rc::SubsystemMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_subsystem.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(SubsystemInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Subsystem,
                    type_id: core_subsystem.get_type_id(),
                    fit_id: core_subsystem.get_fit().get_fit_id(),
                    slot: core_subsystem.get_slot(),
                    state: core_subsystem.get_state(),
                    attrs: get_attrs(core_subsystem, item_mode),
                    effects: get_effects(core_subsystem, item_mode),
                    mods: get_mods(core_subsystem, item_mode),
                }),
            },
        }
    }
}
