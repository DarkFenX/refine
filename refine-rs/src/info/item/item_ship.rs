use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct ShipInfo {
    pub id: rc::ItemId,
    pub extended: Option<ShipInfoExt>,
}

pub struct ShipInfoExt {
    pub kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub state: bool,
    pub coordinates: rc::Coordinates,
    pub movement: rc::Movement,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipInfo {
    pub(super) fn from_core(core_ship: &mut rc::ShipMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_ship.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(ShipInfoExt {
                    kind: rc::ItemKind::Ship,
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
