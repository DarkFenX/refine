use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{AttrId, AttrVals, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification, SlotIndex};

pub struct ImplantInfo {
    pub id: ItemId,
    pub extended: Option<ImplantInfoExt>,
}

pub struct ImplantInfoExt {
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
impl ImplantInfo {
    pub(in crate::info) fn from_core(core_implant: &mut rc::ImplantMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_implant.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(ImplantInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Implant,
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
