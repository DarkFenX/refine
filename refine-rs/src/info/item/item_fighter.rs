use rc::{ItemCommon, Lender};

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AbilityId, AbilityInfo, AttrId, AttrVals, AutochargeInfo, Coordinates, EffectId, EffectInfo, FighterCountInfo,
    FitId, ItemId, ItemInfoMode, ItemRearmMinionInfo, ItemTypeId, MinionState, Modification, Movement, RangedProjInfo,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FighterInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<FighterInfoExt>,
}

#[cfg_attr(feature = "serde", serde_with::serde_as, derive(serde::Serialize))]
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
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub autocharges: Vec<(EffectId, AutochargeInfo)>,
    pub coordinates: Coordinates,
    pub movement: Movement,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub projs: Vec<RangedProjInfo>,
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
impl FighterInfo {
    pub(in crate::info) fn from_core(core_fighter: &mut rc::FighterMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_fighter.get_item_id(),
            extended: match item_mode {
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
                    autocharges: core_fighter
                        .iter_autocharges_mut()
                        .map_into_iter(|mut autocharge| {
                            (
                                autocharge.get_cont_effect_id(),
                                AutochargeInfo::from_core(&mut autocharge, item_mode),
                            )
                        })
                        .collect(),
                    coordinates: core_fighter.get_coordinates(),
                    movement: core_fighter.get_movement(),
                    projs: core_fighter.iter_projs().map(RangedProjInfo::from_core).collect(),
                    attrs: get_attrs(core_fighter, item_mode),
                    effects: get_effects(core_fighter, item_mode),
                    mods: get_mods(core_fighter, item_mode),
                }),
            },
        }
    }
}
