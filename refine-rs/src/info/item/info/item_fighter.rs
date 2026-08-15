use rc::{ItemCommon, Lender};

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AbilityId, AbilityInfo, AttrId, AutochargeInfo, Coordinates, EffectId, FighterCountInfo, FitId, ItemAttrValues,
    ItemEffectInfo, ItemId, ItemInfoMode, ItemRearmMinionInfo, ItemTypeId, MinionState, Modification, Movement,
    RangedProjInfo, info::ItemInfoModesInt,
};

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct FighterInfo {
    pub id: ItemId,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub autocharges: Vec<(EffectId, AutochargeInfo)>,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<FighterInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct FighterInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: MinionState,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub count: Option<FighterCountInfo>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub abilities: Vec<(AbilityId, AbilityInfo)>,
    pub rearm_minion: ItemRearmMinionInfo,
    pub coordinates: Coordinates,
    pub movement: Movement,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub projs: Vec<RangedProjInfo>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub attrs: Vec<(AttrId, ItemAttrValues)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub effects: Vec<(EffectId, ItemEffectInfo)>,
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
impl FighterInfo {
    pub(in crate::info) fn from_core(core_fighter: &mut rc::FighterMut, item_info_modes: &ItemInfoModesInt) -> Self {
        let fighter_id = core_fighter.get_item_id();
        let fighter_info_mode = item_info_modes.get(&fighter_id);
        Self {
            id: fighter_id,
            autocharges: core_fighter
                .iter_autocharges_mut()
                .map_into_iter(|mut autocharge| {
                    (
                        autocharge.get_cont_effect_id(),
                        AutochargeInfo::from_core(&mut autocharge, item_info_modes),
                    )
                })
                .collect(),
            extended: match fighter_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(FighterInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Fighter,
                    type_id: core_fighter.get_type_id(),
                    fit_id: core_fighter.get_fit().get_fit_id(),
                    state: core_fighter.get_state(),
                    count: core_fighter.get_count(),
                    abilities: core_fighter
                        .iter_abilities()
                        .map(|v| (v.get_id(), AbilityInfo::from_core(v)))
                        .collect(),
                    rearm_minion: core_fighter.get_rearm_minion(),
                    coordinates: core_fighter.get_coordinates(),
                    movement: core_fighter.get_movement(),
                    projs: core_fighter.iter_projs().map(RangedProjInfo::from_core).collect(),
                    attrs: get_attrs(core_fighter, fighter_info_mode),
                    effects: get_effects(core_fighter, fighter_info_mode),
                    mods: get_mods(core_fighter, fighter_info_mode),
                }),
            },
        }
    }
}
