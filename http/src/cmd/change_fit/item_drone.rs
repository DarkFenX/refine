use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp, change_item,
        shared::{HMutationOnAdd, apply_mattrs_on_add, get_primary_fit},
    },
    shared::{HCoordinates, HMinionState, HMovement, HNpcProp},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HAddDroneCmd {
    type_id: i32,
    state: HMinionState,
    mutation: Option<HMutationOnAdd>,
    coordinates: Option<HCoordinates>,
    movement: Option<HMovement>,
    prop_mode: Option<HNpcProp>,
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
                    apply_mattrs_on_add(core_mutation, h_full_mutation);
                }
            }
        }
        if let Some(prop_mode) = self.prop_mode {
            core_drone.set_prop_mode(Some(prop_mode.into_core()))
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
    item_cmd: change_item::HChangeDroneCmd,
}
impl HChangeDroneCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
