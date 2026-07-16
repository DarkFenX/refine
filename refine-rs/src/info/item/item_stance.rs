use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{AttrId, AttrVals, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification};

pub struct StanceInfo {
    pub id: ItemId,
    pub extended: Option<StanceInfoExt>,
}

pub struct StanceInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: bool,
    pub attrs: Vec<(AttrId, AttrVals)>,
    pub effects: Vec<(EffectId, EffectInfo)>,
    pub mods: Vec<(AttrId, Vec<Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StanceInfo {
    pub(in crate::info) fn from_core(core_stance: &mut rc::StanceMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_stance.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(StanceInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Stance,
                    type_id: core_stance.get_type_id(),
                    fit_id: core_stance.get_fit().get_fit_id(),
                    state: core_stance.get_state(),
                    attrs: get_attrs(core_stance, item_mode),
                    effects: get_effects(core_stance, item_mode),
                    mods: get_mods(core_stance, item_mode),
                }),
            },
        }
    }
}
