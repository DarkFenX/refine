use crate::{
    cmd::{
        HItemIdsResp, change_item,
        shared::{HMutationOnAdd, apply_mattrs_on_add, get_primary_fit},
    },
    shared::{HCoordinates, HMinionState, HMovement, HNpcProp},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddDroneCmd {
    type_id: rc::ItemTypeId,
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
            self.type_id,
            (&self.state).into(),
            self.coordinates.map(|v| v.into()),
            self.movement.map(|v| v.into()),
            self.prop_mode.unwrap_or(HNpcProp::Chase).into(),
        );
        if let Some(h_mutation) = self.mutation.as_ref() {
            match h_mutation {
                HMutationOnAdd::Short(mutator_id) => {
                    core_drone.mutate(*mutator_id).unwrap();
                }
                HMutationOnAdd::Full(h_full_mutation) => {
                    let core_mutation = core_drone.mutate(h_full_mutation.mutator_id).unwrap();
                    apply_mattrs_on_add(core_mutation, h_full_mutation);
                }
            }
        }
        Ok(core_drone.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeDroneCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeDroneCmd,
}
impl HChangeDroneCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
