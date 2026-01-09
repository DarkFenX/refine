use rc::{ItemCommon, Lender};
use serde::Serialize;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::{
    info::{
        HItemInfoMode,
        item::{
            adj_count::HAdjustableCount, item_autocharge::HAutochargeInfo, item_fighter::ability::HAbilityInfo,
            proj::HRangedProjInfo,
        },
    },
    shared::{HCoordinates, HMinionState, HMovement},
};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HFighterInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    state: HMinionState,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<HAdjustableCount>,
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    abilities: Vec<(i32, HAbilityInfo)>,
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    autocharges: Vec<(rc::EffectId, HAutochargeInfo)>,
    coordinates: HCoordinates,
    movement: HMovement,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    projs: Vec<HRangedProjInfo>,
}
impl HFighterInfoPartial {
    pub(super) fn mk_info(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        Self {
            id: core_fighter.get_item_id(),
            kind: "fighter",
            type_id: core_fighter.get_type_id().into_i32(),
            fit_id: core_fighter.get_fit().get_fit_id(),
            state: HMinionState::from_core(core_fighter.get_state()),
            count: core_fighter.get_count().map(HAdjustableCount::from_core_fighter_count),
            abilities: core_fighter
                .iter_abilities()
                .map(|v| (v.get_id().into_i32(), HAbilityInfo::from_core(v)))
                .collect(),
            autocharges: core_fighter
                .iter_autocharges_mut()
                .map_into_iter(|mut autocharge| {
                    (
                        autocharge.get_cont_effect_id().into(),
                        HAutochargeInfo::mk_info(&mut autocharge, item_mode),
                    )
                })
                .collect(),
            coordinates: HCoordinates::from_core(core_fighter.get_coordinates()),
            movement: HMovement::from_core(core_fighter.get_movement()),
            projs: core_fighter.iter_projs().map(HRangedProjInfo::from_core).collect(),
        }
    }
}
