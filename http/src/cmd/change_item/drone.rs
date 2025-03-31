use crate::{
    cmd::{
        HCmdResp,
        shared::{HEffectModeMap, HMutationOnChange, HProjDef, HProjDefFull, apply_effect_modes},
    },
    shared::HMinionState,
    util::{HExecError, TriStateField},
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeDroneCmd {
    #[serde(default)]
    state: Option<HMinionState>,
    #[serde(default)]
    mutation: TriStateField<HMutationOnChange>,
    #[serde(default)]
    add_projs: Vec<HProjDef>,
    #[serde(default)]
    change_projs: Vec<HProjDefFull>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::ItemId>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeDroneCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HCmdResp, HExecError> {
        if let Some(state) = &self.state {
            if let Err(error) = core_sol.set_drone_state(item_id, state.into()) {
                return Err(match error {
                    rc::err::SetDroneStateError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::SetDroneStateError::ItemIsNotDrone(e) => HExecError::ItemKindMismatch(e),
                });
            }
        }
        match &self.mutation {
            TriStateField::Value(mutation) => match mutation {
                HMutationOnChange::AddShort(mutator_id) => {
                    // Remove old mutation if we had any, ignore any errors on the way
                    let _ = core_sol.remove_drone_mutation(item_id);
                    let mutation = rc::ItemAddMutation::new(*mutator_id);
                    if let Err(error) = core_sol.add_drone_mutation(item_id, mutation) {
                        match error {
                            rc::err::AddDroneMutationError::ItemNotFound(e) => {
                                return Err(HExecError::ItemNotFoundPrimary(e));
                            }
                            rc::err::AddDroneMutationError::ItemIsNotDrone(e) => {
                                return Err(HExecError::ItemKindMismatch(e));
                            }
                            rc::err::AddDroneMutationError::MutationAlreadySet(_) => {
                                panic!("no mutation should be set")
                            }
                        };
                    }
                }
                HMutationOnChange::AddFull(mutation) => {
                    // Remove old mutation if we had any, ignore any errors on the way
                    let _ = core_sol.remove_drone_mutation(item_id);
                    if let Err(error) = core_sol.add_drone_mutation(item_id, mutation.into()) {
                        match error {
                            rc::err::AddDroneMutationError::ItemNotFound(e) => {
                                return Err(HExecError::ItemNotFoundPrimary(e));
                            }
                            rc::err::AddDroneMutationError::ItemIsNotDrone(e) => {
                                return Err(HExecError::ItemKindMismatch(e));
                            }
                            rc::err::AddDroneMutationError::MutationAlreadySet(_) => {
                                panic!("no mutation should be set")
                            }
                        };
                    }
                }
                HMutationOnChange::ChangeAttrs(attr_mutations) => {
                    let attr_mutations = attr_mutations
                        .iter()
                        .map(|(k, v)| rc::ItemChangeAttrMutation::new(*k, v.as_ref().map(|v| v.into())))
                        .collect();
                    if let Err(error) = core_sol.change_drone_mutation(item_id, attr_mutations) {
                        return Err(match error {
                            rc::err::ChangeDroneMutationError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                            rc::err::ChangeDroneMutationError::ItemIsNotDrone(e) => HExecError::ItemKindMismatch(e),
                            rc::err::ChangeDroneMutationError::MutationNotSet(e) => HExecError::MutationNotSet(e),
                        });
                    }
                }
            },
            TriStateField::None => {
                if let Err(error) = core_sol.remove_drone_mutation(item_id) {
                    match error {
                        rc::err::RemoveDroneMutationError::ItemNotFound(e) => {
                            return Err(HExecError::ItemNotFoundPrimary(e));
                        }
                        rc::err::RemoveDroneMutationError::ItemIsNotDrone(e) => {
                            return Err(HExecError::ItemKindMismatch(e));
                        }
                        // Do nothing if mutation was not there
                        rc::err::RemoveDroneMutationError::MutationNotSet(_) => (),
                    };
                };
            }
            TriStateField::Absent => (),
        }
        for proj_def in self.add_projs.iter() {
            if let Err(error) = core_sol.add_drone_proj(item_id, proj_def.get_item_id(), proj_def.get_range()) {
                return Err(match error {
                    rc::err::AddDroneProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::AddDroneProjError::ProjectorIsNotDrone(e) => HExecError::ItemKindMismatch(e),
                    rc::err::AddDroneProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddDroneProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddDroneProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
                });
            }
        }
        for proj_def in self.change_projs.iter() {
            if let Err(error) = core_sol.change_drone_proj(item_id, &proj_def.get_item_id(), proj_def.get_range()) {
                return Err(match error {
                    rc::err::ChangeDroneProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::ChangeDroneProjError::ProjectorIsNotDrone(e) => HExecError::ItemKindMismatch(e),
                    rc::err::ChangeDroneProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                });
            }
        }
        for projectee_item_id in self.rm_projs.iter() {
            if let Err(error) = core_sol.remove_drone_proj(item_id, projectee_item_id) {
                return Err(match error {
                    rc::err::RemoveDroneProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::RemoveDroneProjError::ProjectorIsNotDrone(e) => HExecError::ItemKindMismatch(e),
                    rc::err::RemoveDroneProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                });
            }
        }
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
