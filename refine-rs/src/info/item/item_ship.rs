use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, AttrVals, Coordinates, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification,
    Movement,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ShipInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<ShipInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
pub struct ShipInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: bool,
    pub coordinates: Coordinates,
    pub movement: Movement,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub attrs: Vec<(AttrId, AttrVals)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub effects: Vec<(EffectId, EffectInfo)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub mods: Vec<(AttrId, Vec<Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipInfo {
    pub(in crate::info) fn from_core(core_ship: &mut rc::ShipMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_ship.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(ShipInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Ship,
                    type_id: core_ship.get_type_id(),
                    fit_id: core_ship.get_fit().get_fit_id(),
                    state: core_ship.get_state(),
                    coordinates: core_ship.get_coordinates(),
                    movement: core_ship.get_movement(),
                    attrs: get_attrs(core_ship, item_mode),
                    effects: get_effects(core_ship, item_mode),
                    mods: get_mods(core_ship, item_mode),
                }),
            },
        }
    }
}
