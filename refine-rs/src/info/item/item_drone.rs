use rc::ItemCommon;

use super::shared::{ItemMutationInfo, RangedProjInfo, get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct DroneInfo {
    pub id: rc::ItemId,
    pub extended: Option<DroneInfoExt>,
}

pub struct DroneInfoExt {
    #[cfg(feature = "serde")]
    kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub state: rc::MinionState,
    pub mutation: Option<ItemMutationInfo>,
    pub npc_prop: rc::ItemNpcPropInfo,
    pub coordinates: rc::Coordinates,
    pub movement: rc::Movement,
    pub projs: Vec<RangedProjInfo>,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DroneInfo {
    pub(in crate::info) fn from_core(core_drone: &mut rc::DroneMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_drone.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(DroneInfoExt {
                    #[cfg(feature = "serde")]
                    kind: rc::ItemKind::Drone,
                    type_id: core_drone.get_type_id(),
                    fit_id: core_drone.get_fit().get_fit_id(),
                    state: core_drone.get_state(),
                    mutation: core_drone.get_mutation().and_then(ItemMutationInfo::try_from_core),
                    npc_prop: core_drone.get_npc_prop(),
                    coordinates: core_drone.get_coordinates(),
                    movement: core_drone.get_movement(),
                    projs: core_drone.iter_projs().map(RangedProjInfo::from_core).collect(),
                    attrs: get_attrs(core_drone, item_mode),
                    effects: get_effects(core_drone, item_mode),
                    mods: get_mods(core_drone, item_mode),
                }),
            },
        }
    }
}
