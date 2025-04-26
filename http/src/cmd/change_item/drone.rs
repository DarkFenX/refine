use crate::{
    cmd::{
        HItemIdsResp,
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
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_drone = core_sol.get_drone_mut(item_id).map_err(|error| match error {
            rc::err::GetDroneError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetDroneError::ItemIsNotDrone(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(state) = &self.state {
            core_drone.set_state(state.into());
        }
        match &self.mutation {
            TriStateField::Value(mutation) => match mutation {
                HMutationOnChange::AddShort(mutator_id) => {
                    // Remove old mutation if we had any, ignore any errors on the way
                    let _ = core_drone.unmutate();
                    let mutation = rc::ItemAddMutation::new(*mutator_id);
                    core_drone.mutate(mutation).unwrap();
                }
                HMutationOnChange::AddFull(mutation) => {
                    // Remove old mutation if we had any, ignore any errors on the way
                    let _ = core_drone.unmutate();
                    core_drone.mutate(mutation.into()).unwrap();
                }
                HMutationOnChange::ChangeAttrs(attr_mutations) => {
                    let attr_mutations = attr_mutations
                        .iter()
                        .map(|(k, v)| rc::ItemChangeAttrMutation::new(*k, v.as_ref().map(|v| v.into())))
                        .collect();
                    core_drone
                        .change_mutation(attr_mutations)
                        .map_err(|error| match error {
                            rc::err::ChangeDroneMutationError::MutationNotSet(e) => HExecError::MutationNotSet(e),
                        })?;
                }
            },
            TriStateField::None => {
                // Do nothing if mutation was not there
                let _ = core_drone.unmutate();
            }
            TriStateField::Absent => (),
        }
        for proj_def in self.add_projs.iter() {
            core_drone
                .add_proj(&proj_def.get_item_id(), proj_def.get_range())
                .map_err(|error| match error {
                    rc::err::AddDroneProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddDroneProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddDroneProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
                })?;
        }
        for proj_def in self.change_projs.iter() {
            core_drone
                .get_proj_mut(&proj_def.get_item_id())
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .set_range(proj_def.get_range());
        }
        for projectee_item_id in self.rm_projs.iter() {
            core_drone
                .get_proj_mut(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .remove();
        }
        apply_effect_modes(&mut core_drone, &self.effect_modes);
        Ok(core_drone.into())
    }
}
