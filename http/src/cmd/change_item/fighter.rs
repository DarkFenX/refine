use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HAbilityMap, HEffectModeMap, apply_abilities, apply_effect_modes},
    },
    shared::{HCoordinates, HMinionState, HMovement},
    util::{HExecError, TriStateField},
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    #[serde(default)]
    type_id: Option<rc::ItemTypeId>,
    #[serde(default)]
    state: Option<HMinionState>,
    #[serde(default)]
    count: TriStateField<rc::Count>,
    #[serde(default)]
    abilities: Option<HAbilityMap>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    add_projs: Vec<rc::ItemId>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::ItemId>,
    #[serde(default)]
    coordinates: Option<HCoordinates>,
    #[serde(default)]
    movement: Option<HMovement>,
    #[serde(default)]
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
            core_fighter.set_type_id(type_id);
        }
        if let Some(state) = &self.state {
            core_fighter.set_state(state.into());
        }
        match self.count {
            TriStateField::Value(count) => {
                let fighter_count_override = rc::FighterCountOverride::new_checked(count)?;
                core_fighter.set_count_override(Some(fighter_count_override));
            }
            TriStateField::None => {
                core_fighter.set_count_override(None);
            }
            TriStateField::Absent => (),
        }
        apply_abilities(&mut core_fighter, &self.abilities);
        for projectee_item_id in self.rm_projs.iter() {
            core_fighter
                .get_proj_mut(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .remove();
        }
        if let Some(coordinates) = self.coordinates {
            core_fighter.set_coordinates(coordinates.into());
        }
        if let Some(movement) = self.movement {
            core_fighter.set_movement(movement.into());
        }
        for projectee_item_id in self.add_projs.iter() {
            core_fighter.add_proj(projectee_item_id).map_err(|error| match error {
                rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
            })?;
        }
        apply_effect_modes(&mut core_fighter, &self.effect_modes);
        Ok(core_fighter.into())
    }
}
