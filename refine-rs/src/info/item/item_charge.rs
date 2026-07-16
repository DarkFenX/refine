use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{AttrId, AttrVals, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification};

pub struct ChargeInfo {
    pub id: ItemId,
    pub extended: Option<ChargeInfoExt>,
}

pub struct ChargeInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub cont_item_id: ItemId,
    pub state: bool,
    pub attrs: Vec<(AttrId, AttrVals)>,
    pub effects: Vec<(EffectId, EffectInfo)>,
    pub mods: Vec<(AttrId, Vec<Modification>)>,
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
                    kind: ItemKind::Charge,
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
