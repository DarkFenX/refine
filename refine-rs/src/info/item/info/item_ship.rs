use rc::ItemCommon;

#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    Coordinates, EffectId, EffectMode, FitId, ItemId, ItemInfoFull, ItemInfoMode, ItemTypeId, Movement,
    shared::OvrdMapLight,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct ShipInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<ShipInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
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
    pub effect_mode_overrides: Vec<(EffectId, EffectMode)>,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub full: Option<ItemInfoFull>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipInfo {
    pub(in crate::info) fn from_core(
        core_ship: &mut rc::Ship,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let ship_id = core_ship.get_item_id();
        let ship_info_mode = item_info_modes.get(&ship_id);
        Self {
            id: ship_id,
            extended: match ship_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(ShipInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Ship,
                    type_id: core_ship.get_type_id(),
                    fit_id: core_ship.get_fit_id(),
                    state: core_ship.get_state(),
                    coordinates: core_ship.get_coordinates(),
                    movement: core_ship.get_movement(),
                    effect_mode_overrides: core_ship.iter_effect_mode_overrides().collect(),
                    full: ItemInfoFull::try_from_core(core_ship, ship_info_mode),
                }),
            },
        }
    }
}
