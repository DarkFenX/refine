use rc::{ItemCommon, Lender};

use super::shared::{AbilityInfo, RangedProjInfo, get_attrs, get_effects, get_mods};
use crate::info::{AutochargeInfo, ItemInfoMode};

pub struct FighterInfo {
    pub id: rc::ItemId,
    pub extended: Option<FighterInfoExt>,
}

pub struct FighterInfoExt {
    kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub state: rc::MinionState,
    pub count: Option<rc::FighterCountInfo>,
    pub abilities: Vec<(rc::AbilityId, AbilityInfo)>,
    pub rearm_minion: rc::ItemRearmMinionInfo,
    pub autocharges: Vec<(rc::EffectId, AutochargeInfo)>,
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
impl FighterInfo {
    pub(in crate::info) fn from_core(core_fighter: &mut rc::FighterMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_fighter.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(FighterInfoExt {
                    kind: rc::ItemKind::Fighter,
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
