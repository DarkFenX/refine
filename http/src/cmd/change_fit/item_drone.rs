use crate::{
    cmd::{
        HItemIdsResp, change_item,
        shared::{HMutationOnAdd, get_primary_fit},
    },
    shared::HMinionState,
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddDroneCmd {
    type_id: rc::ItemTypeId,
    state: HMinionState,
    mutation: Option<HMutationOnAdd>,
}
impl HAddDroneCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let core_drone = core_fit.add_drone(
            self.type_id,
            (&self.state).into(),
            self.mutation.as_ref().map(|v| v.into()),
        );
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
