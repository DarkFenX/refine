use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, HMutationOnChange},
    },
    shared::{HCoordinates, HMinionState, HMovement, HNpcProp},
    util::{HExecError, TriStateField},
};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeDroneCmd {
    type_id: Option<i32>,
    state: Option<HMinionState>,
    #[serde(default)]
    mutation: TriStateField<HMutationOnChange>,
    #[serde(default)]
    npc_prop: TriStateField<HNpcProp>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    add_proj_item_ids: Vec<rc::ItemId>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    rm_proj_item_ids: Vec<rc::ItemId>,
    coordinates: Option<HCoordinates>,
    movement: Option<HMovement>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeDroneCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_drone = core_sol.get_drone_mut(item_id).map_err(|error| match error {
            rc::err::GetDroneError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetDroneError::ItemIsNotDrone(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_drone.set_type_id(core_type_id);
        }
        if let Some(state) = &self.state {
            core_drone.set_state(state.into_core());
        }
        match &self.mutation {
            TriStateField::Value(mutation) => match mutation {
                // Mutates item or updates existing mutation
                HMutationOnChange::Mutator(mutator_id) => {
                    let core_mutator_id = rc::ItemTypeId::from_i32(*mutator_id);
                    match core_drone.get_mutation_mut() {
                        Some(mutation) => mutation.set_mutator_type_id(core_mutator_id),
                        None => core_drone.mutate(core_mutator_id).unwrap(),
                    };
                }
                // Updates existing mutation
                HMutationOnChange::Attrs(h_attr_mutations) => {
                    let Some(core_mutation) = core_drone.get_mutation_mut() else {
                        return Err(HExecError::MutationNotSet(*item_id));
                    };
                    h_attr_mutations.apply(core_mutation);
                }
                // Mutates item, or overwrites mutation, if it was set
                HMutationOnChange::MutatorAndAttrs(h_full_mutation) => {
                    if let Some(core_mutation) = core_drone.get_mutation_mut() {
                        core_mutation.remove();
                    }
                    let core_mutator_id = rc::ItemTypeId::from_i32(h_full_mutation.mutator_id);
                    let core_mutation = core_drone.mutate(core_mutator_id).unwrap();
                    h_full_mutation.apply_attrs_on_add(core_mutation);
                }
            },
            TriStateField::None => {
                // Do nothing if mutation was not there
                if let Some(core_mutation) = core_drone.get_mutation_mut() {
                    core_mutation.remove();
                }
            }
            TriStateField::Absent => (),
        }
        match self.npc_prop {
            TriStateField::Value(h_npc_prop) => core_drone.set_npc_prop(Some(h_npc_prop.into_core())),
            TriStateField::None => core_drone.set_npc_prop(None),
            TriStateField::Absent => (),
        }
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_drone
                .get_proj_mut(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .remove();
        }
        if let Some(coordinates) = self.coordinates {
            core_drone.set_coordinates(coordinates.into_core());
        }
        if let Some(movement) = self.movement {
            core_drone.set_movement(movement.into_core());
        }
        for projectee_item_id in self.add_proj_item_ids.iter() {
            core_drone.add_proj(projectee_item_id).map_err(|error| match error {
                rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
            })?;
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_drone);
        }
        Ok(HItemIdsResp::from_core_drone(core_drone))
    }
}
