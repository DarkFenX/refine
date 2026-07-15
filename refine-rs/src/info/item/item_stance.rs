use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct StanceInfo {
    pub id: rc::ItemId,
    pub extended: Option<StanceInfoExt>,
}

pub struct StanceInfoExt {
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
impl StanceInfo {
    pub(super) fn from_core(core_stance: &mut rc::StanceMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_stance.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(StanceInfoExt {
                    kind: rc::ItemKind::Stance,
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
