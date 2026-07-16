use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct SubsystemInfo {
    pub id: rc::ItemId,
    pub extended: Option<SubsystemInfoExt>,
}

pub struct SubsystemInfoExt {
    pub kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub slot: Option<rc::SlotIndex>,
    pub state: bool,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
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
                    kind: rc::ItemKind::Subsystem,
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
