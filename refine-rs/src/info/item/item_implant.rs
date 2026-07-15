use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct ImplantInfo {
    pub id: rc::ItemId,
    pub extended: Option<ImplantInfoExt>,
}

pub struct ImplantInfoExt {
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
impl ImplantInfo {
    pub(super) fn from_core(core_implant: &mut rc::ImplantMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_implant.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(ImplantInfoExt {
                    kind: rc::ItemKind::Implant,
                    type_id: core_implant.get_type_id(),
                    fit_id: core_implant.get_fit().get_fit_id(),
                    slot: core_implant.get_slot(),
                    state: core_implant.get_state(),
                    attrs: get_attrs(core_implant, item_mode),
                    effects: get_effects(core_implant, item_mode),
                    mods: get_mods(core_implant, item_mode),
                }),
            },
        }
    }
}
