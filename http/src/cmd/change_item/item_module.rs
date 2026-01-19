use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, HMutationOnChange, apply_effect_modes, apply_mattrs_on_add, apply_mattrs_on_change},
    },
    shared::{HModuleState, HSpool},
    util::{HExecError, TriStateField},
};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde(default)]
    type_id: Option<i32>,
    #[serde(default)]
    state: Option<HModuleState>,
    #[serde(default)]
    mutation: TriStateField<HMutationOnChange>,
    #[serde(default)]
    charge_type_id: TriStateField<i32>,
    #[serde(default)]
    spool: TriStateField<HSpool>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    add_projs: Vec<rc::ItemId>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::ItemId>,
    #[serde(default)]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_module = core_sol.get_module_mut(item_id).map_err(|error| match error {
            rc::err::GetModuleError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetModuleError::ItemIsNotModule(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_module.set_type_id(core_type_id);
        }
        if let Some(state) = &self.state {
            core_module.set_state(state.into_core());
        }
        match &self.mutation {
            TriStateField::Value(mutation) => match mutation {
                // Mutates item or updates existing mutation
                HMutationOnChange::Mutator(mutator_id) => {
                    let core_mutator_id = rc::ItemTypeId::from_i32(*mutator_id);
                    match core_module.get_mutation_mut() {
                        Some(mutation) => mutation.set_mutator_type_id(core_mutator_id),
                        None => core_module.mutate(core_mutator_id).unwrap(),
                    };
                }
                // Updates existing mutation
                HMutationOnChange::Attrs(h_attr_mutations) => {
                    let core_mutation = match core_module.get_mutation_mut() {
                        Some(core_mutation) => core_mutation,
                        None => return Err(HExecError::MutationNotSet(*item_id)),
                    };
                    apply_mattrs_on_change(core_mutation, h_attr_mutations);
                }
                // Mutates item, or overwrites mutation, if it was set
                HMutationOnChange::MutatorAndAttrs(mutation) => {
                    if let Some(core_mutation) = core_module.get_mutation_mut() {
                        core_mutation.remove();
                    }
                    let core_mutator_id = rc::ItemTypeId::from_i32(mutation.mutator_id);
                    let core_mutation = core_module.mutate(core_mutator_id).unwrap();
                    apply_mattrs_on_add(core_mutation, mutation);
                }
            },
            TriStateField::None => {
                // Do nothing if mutation was not there
                if let Some(core_mutation) = core_module.get_mutation_mut() {
                    core_mutation.remove();
                }
            }
            TriStateField::Absent => (),
        }
        match &self.charge_type_id {
            TriStateField::Value(charge_type_id) => {
                let core_charge_type_id = rc::ItemTypeId::from_i32(*charge_type_id);
                core_module.set_charge_type_id(core_charge_type_id);
            }
            TriStateField::None => match core_module.get_charge_mut() {
                Some(core_charge) => core_charge.remove(),
                None => return Err(HExecError::ChargeNotSet(*item_id)),
            },
            TriStateField::Absent => (),
        }
        match self.spool {
            TriStateField::Value(h_spool) => core_module.set_spool(Some(h_spool.into_core())),
            TriStateField::None => core_module.set_spool(None),
            TriStateField::Absent => (),
        }
        for projectee_item_id in self.add_projs.iter() {
            core_module.add_proj(projectee_item_id).map_err(|error| match error {
                rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
            })?;
        }
        for projectee_item_id in self.rm_projs.iter() {
            core_module
                .get_proj_mut(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .remove();
        }
        apply_effect_modes(&mut core_module, &self.effect_modes);
        Ok(HItemIdsResp::from_core_module(core_module))
    }
}
