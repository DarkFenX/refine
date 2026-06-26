use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp, old_change_item,
        shared::{HEffectModeMap, HMutationOnAdd, get_primary_fit},
    },
    shared::{HCoordinates, HMinionState, HMovement, HNpcProp},
    util::HExecError,
};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HAddDroneCmd {
    type_id: i32,
    state: HMinionState,
    mutation: Option<HMutationOnAdd>,
    npc_prop: Option<HNpcProp>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    proj_item_ids: Vec<rc::ItemId>,
    coordinates: Option<HCoordinates>,
    movement: Option<HMovement>,
    effect_modes: Option<HEffectModeMap>,
}
impl HAddDroneCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_drone = core_fit.add_drone(
            rc::ItemTypeId::from_i32(self.type_id),
            self.state.into_core(),
            self.coordinates.map(|v| v.into_core()),
            self.movement.map(|v| v.into_core()),
        );
        if let Some(h_mutation) = self.mutation.as_ref() {
            match h_mutation {
                HMutationOnAdd::Short(mutator_id) => {
                    let core_mutator_id = rc::ItemTypeId::from_i32(*mutator_id);
                    core_drone.mutate(core_mutator_id).unwrap();
                }
                HMutationOnAdd::Full(h_full_mutation) => {
                    let core_mutator_id = rc::ItemTypeId::from_i32(h_full_mutation.mutator_id);
                    let core_mutation = core_drone.mutate(core_mutator_id).unwrap();
                    h_full_mutation.apply_attrs_on_add(core_mutation);
                }
            }
        }
        if let Some(h_npc_prop) = self.npc_prop {
            core_drone.set_npc_prop(Some(h_npc_prop.into_core()))
        }
        for projectee_item_id in self.proj_item_ids.iter() {
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

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeDroneCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: old_change_item::HChangeDroneCmdComplete,
}
impl HChangeDroneCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
