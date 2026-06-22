use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HAbilityMap, HEffectModeMap},
    },
    shared::{HCoordinates, HMinionState, HMovement, HRearmMinion},
    util::{HExecError, TriStateField},
};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeFighterCmd {
    type_id: Option<i32>,
    state: Option<HMinionState>,
    #[serde(default)]
    count: TriStateField<u32>,
    abilities: Option<HAbilityMap>,
    #[serde(default)]
    rearm_minion: TriStateField<HRearmMinion>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    add_projs: Vec<rc::ItemId>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::ItemId>,
    coordinates: Option<HCoordinates>,
    movement: Option<HMovement>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeFighterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fighter = core_sol.get_fighter_mut(item_id).map_err(|error| match error {
            rc::err::GetFighterError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetFighterError::ItemIsNotFighter(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            core_fighter.set_type_id(rc::ItemTypeId::from_i32(type_id));
        }
        if let Some(state) = &self.state {
            core_fighter.set_state(state.into_core());
        }
        match self.count {
            TriStateField::Value(count) => {
                let fighter_count_override = rc::FighterCount::from_u32_checked(count)?;
                core_fighter.set_count_override(Some(fighter_count_override));
            }
            TriStateField::None => {
                core_fighter.set_count_override(None);
            }
            TriStateField::Absent => (),
        }
        if let Some(abilities) = self.abilities.as_ref() {
            abilities.apply(&mut core_fighter);
        }
        match self.rearm_minion {
            TriStateField::Value(h_rearm_minion) => core_fighter.set_rearm_minion(Some(h_rearm_minion.into_core())),
            TriStateField::None => core_fighter.set_rearm_minion(None),
            TriStateField::Absent => (),
        }
        for projectee_item_id in self.rm_projs.iter() {
            core_fighter
                .get_proj_mut(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .remove();
        }
        for projectee_item_id in self.add_projs.iter() {
            core_fighter.add_proj(projectee_item_id).map_err(|error| match error {
                rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
            })?;
        }
        if let Some(coordinates) = self.coordinates {
            core_fighter.set_coordinates(coordinates.into_core());
        }
        if let Some(movement) = self.movement {
            core_fighter.set_movement(movement.into_core());
        }
        if let Some(effect_modes) = self.effect_modes.as_ref() {
            effect_modes.apply(&mut core_fighter);
        }
        Ok(HItemIdsResp::from_core_fighter(core_fighter))
    }
}
