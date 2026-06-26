use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp, old_change_item,
        shared::{HAddMode, HEffectModeMap, HMutationOnAdd, get_primary_fit},
    },
    shared::{HModRack, HModuleState, HOptionalReload, HSpool},
    util::HExecError,
};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HAddModuleCmd {
    rack: HModRack,
    add_mode: HAddMode,
    type_id: i32,
    state: HModuleState,
    mutation: Option<HMutationOnAdd>,
    charge_type_id: Option<i32>,
    spool: Option<HSpool>,
    optional_reload: Option<HOptionalReload>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    proj_item_ids: Vec<rc::ItemId>,
    effect_modes: Option<HEffectModeMap>,
}
impl HAddModuleCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_module = core_fit.add_module(
            self.rack.into_core(),
            self.add_mode.into_core(),
            rc::ItemTypeId::from_i32(self.type_id),
            self.state.into_core(),
        );
        if let Some(h_mutation) = self.mutation.as_ref() {
            match h_mutation {
                HMutationOnAdd::Short(mutator_id) => {
                    let core_mutator_id = rc::ItemTypeId::from_i32(*mutator_id);
                    core_module.mutate(core_mutator_id).unwrap();
                }
                HMutationOnAdd::Full(h_full_mutation) => {
                    let core_mutator_id = rc::ItemTypeId::from_i32(h_full_mutation.mutator_id);
                    let core_mutation = core_module.mutate(core_mutator_id).unwrap();
                    h_full_mutation.apply_attrs_on_add(core_mutation);
                }
            }
        }
        if let Some(charge_type_id) = self.charge_type_id {
            let core_charge_type_id = rc::ItemTypeId::from_i32(charge_type_id);
            core_module.set_charge_type_id(core_charge_type_id);
        }
        if let Some(h_spool) = self.spool {
            core_module.set_spool(Some(h_spool.into_core()));
        }
        if let Some(h_optional_reload) = self.optional_reload {
            core_module.set_optional_reload(Some(h_optional_reload.into_core()));
        }
        for projectee_item_id in self.proj_item_ids.iter() {
            core_module.add_proj(projectee_item_id).map_err(|error| match error {
                rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
            })?;
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_module);
        }
        Ok(HItemIdsResp::from_core_module(core_module))
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: old_change_item::HChangeModuleCmd,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
