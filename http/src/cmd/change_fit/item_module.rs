use crate::{
    cmd::{
        HItemIdsResp, change_item,
        shared::{HAddMode, HMutationOnAdd, apply_mattrs_on_add, get_primary_fit},
    },
    shared::{HModRack, HModuleState, HSpool},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    rack: HModRack,
    add_mode: HAddMode,
    type_id: rc::ItemTypeId,
    state: HModuleState,
    mutation: Option<HMutationOnAdd>,
    charge_type_id: Option<rc::ItemTypeId>,
    spool: Option<HSpool>,
}
impl HAddModuleCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_module = core_fit.add_module(
            (&self.rack).into(),
            (&self.add_mode).into(),
            self.type_id,
            (&self.state).into(),
        );
        if let Some(h_mutation) = self.mutation.as_ref() {
            match h_mutation {
                HMutationOnAdd::Short(mutator_id) => {
                    core_module.mutate(*mutator_id).unwrap();
                }
                HMutationOnAdd::Full(h_full_mutation) => {
                    let core_mutation = core_module.mutate(h_full_mutation.mutator_id).unwrap();
                    apply_mattrs_on_add(core_mutation, h_full_mutation);
                }
            }
        }
        if let Some(charge_type_id) = self.charge_type_id {
            core_module.set_charge_type_id(charge_type_id);
        }
        if let Some(h_spool) = self.spool {
            core_module.set_spool(Some(h_spool.into()));
        }
        Ok(core_module.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeModuleCmd,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
