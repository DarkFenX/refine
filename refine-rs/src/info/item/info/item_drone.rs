use rc::{ItemCommon, Lender};

#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    Coordinates, EffectId, EffectMode, FitId, ItemId, ItemInfoFull, ItemInfoMode, ItemMutationInfo, ItemTypeId,
    MinionState, Movement, NpcProp, RangedProjInfo, shared::OvrdMapLight,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct DroneInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<DroneInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct DroneInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: MinionState,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mutation: Option<ItemMutationInfo>,
    pub npc_prop: NpcProp,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub npc_prop_override: Option<NpcProp>,
    pub coordinates: Coordinates,
    pub movement: Movement,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub projs: Vec<RangedProjInfo>,
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
impl DroneInfo {
    pub(in crate::info) fn from_core(
        core_drone: &mut rc::Drone,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let drone_id = core_drone.get_item_id();
        let drone_info_mode = item_info_modes.get(&drone_id);
        Self {
            id: drone_id,
            extended: match drone_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(DroneInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Drone,
                    type_id: core_drone.get_type_id(),
                    fit_id: core_drone.get_fit_id(),
                    state: core_drone.get_state(),
                    mutation: core_drone.get_mutation().map(ItemMutationInfo::try_from_core),
                    npc_prop: core_drone.get_npc_prop(),
                    npc_prop_override: core_drone.get_npc_prop_override(),
                    coordinates: core_drone.get_coordinates(),
                    movement: core_drone.get_movement(),
                    projs: core_drone
                        .iter_projs()
                        .map_into_iter(RangedProjInfo::from_core)
                        .collect(),
                    effect_mode_overrides: core_drone.iter_effect_mode_overrides().collect(),
                    full: ItemInfoFull::try_from_core(core_drone, drone_info_mode),
                }),
            },
        }
    }
}
