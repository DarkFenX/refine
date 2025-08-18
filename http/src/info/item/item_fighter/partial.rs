use std::collections::HashMap;

use rc::{ItemCommon, Lender};

use crate::{
    info::{
        HItemInfoMode,
        item::{
            adj_count::HAdjustableCount, item_autocharge::HAutochargeInfo, item_fighter::ability::HAbilityInfo,
            proj::HRangedProjInfo,
        },
    },
    shared::{HCoordinates, HEffectId, HMinionState, HMovement},
};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
    state: HMinionState,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<HAdjustableCount>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    abilities: HashMap<rc::AbilId, HAbilityInfo>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    autocharges: HashMap<HEffectId, HAutochargeInfo>,
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
            type_id: core_fighter.get_type_id(),
            fit_id: core_fighter.get_fit().get_fit_id(),
            state: (&core_fighter.get_state()).into(),
            count: core_fighter.get_count().map(|v| v.into()),
            abilities: core_fighter.iter_abilities().map(|v| (v.get_id(), v.into())).collect(),
            autocharges: core_fighter
                .iter_autocharges_mut()
                .map_into_iter(|mut autocharge| {
                    (
                        autocharge.get_cont_effect_id().into(),
                        HAutochargeInfo::mk_info(&mut autocharge, item_mode),
                    )
                })
                .collect(),
            coordinates: core_fighter.get_coordinates().into(),
            movement: core_fighter.get_movement().into(),
            projs: core_fighter
                .iter_projs()
                .map(|core_ranged_proj| core_ranged_proj.into())
                .collect(),
        }
    }
}
