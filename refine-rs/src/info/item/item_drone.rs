use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, AttrVals, Coordinates, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemMutationInfo,
    ItemNpcPropInfo, ItemTypeId, MinionState, Modification, Movement, RangedProjInfo,
};

pub struct DroneInfo {
    pub id: ItemId,
    pub extended: Option<DroneInfoExt>,
}

pub struct DroneInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: MinionState,
    pub mutation: Option<ItemMutationInfo>,
    pub npc_prop: ItemNpcPropInfo,
    pub coordinates: Coordinates,
    pub movement: Movement,
    pub projs: Vec<RangedProjInfo>,
    pub attrs: Vec<(AttrId, AttrVals)>,
    pub effects: Vec<(EffectId, EffectInfo)>,
    pub mods: Vec<(AttrId, Vec<Modification>)>,
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
                    kind: ItemKind::Drone,
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
