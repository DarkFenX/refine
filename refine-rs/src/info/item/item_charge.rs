use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct ChargeInfo {
    pub id: rc::ItemId,
    pub extended: Option<ChargeInfoExt>,
}

pub struct ChargeInfoExt {
    #[cfg(feature = "serde")]
    kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub cont_item_id: rc::ItemId,
    pub state: bool,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChargeInfo {
    pub(super) fn from_core(core_charge: &mut rc::ChargeMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_charge.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(ChargeInfoExt {
                    #[cfg(feature = "serde")]
                    kind: rc::ItemKind::Charge,
                    type_id: core_charge.get_type_id(),
                    fit_id: core_charge.get_fit().get_fit_id(),
                    cont_item_id: core_charge.get_cont_item().get_item_id(),
                    state: core_charge.get_state(),
                    attrs: get_attrs(core_charge, item_mode),
                    effects: get_effects(core_charge, item_mode),
                    mods: get_mods(core_charge, item_mode),
                }),
            },
        }
    }
}
