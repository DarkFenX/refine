use crate::{
    cmd::{
        HItemIdsResp,
        shared::{
            HEffectModeMap, HMutationOnChange, HProjDef, HProjDefFull, apply_effect_modes, apply_mattrs_on_add,
            apply_mattrs_on_change,
        },
    },
    shared::HModuleState,
    util::{HExecError, TriStateField},
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    state: Option<HModuleState>,
    #[serde(default)]
    mutation: TriStateField<HMutationOnChange>,
    #[serde(default)]
    charge: TriStateField<rc::ItemTypeId>,
    #[serde(default)]
    add_projs: Vec<HProjDef>,
    #[serde(default)]
    change_projs: Vec<HProjDefFull>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::ItemId>,
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
        if let Some(state) = &self.state {
            core_module.set_state(state.into());
        }
        match &self.mutation {
            TriStateField::Value(mutation) => match mutation {
                HMutationOnChange::AddShort(mutator_id) => {
                    // Remove old mutation if we had any
                    if let Some(core_mutation) = core_module.get_mutation_mut() {
                        core_mutation.remove();
                    }
                    core_module.mutate(*mutator_id).unwrap();
                }
                HMutationOnChange::AddFull(mutation) => {
                    // Remove old mutation if we had any
                    if let Some(core_mutation) = core_module.get_mutation_mut() {
                        core_mutation.remove();
                    }
                    let core_mutation = core_module.mutate(mutation.mutator_id).unwrap();
                    apply_mattrs_on_add(core_mutation, mutation);
                }
                HMutationOnChange::ChangeAttrs(h_attr_mutations) => {
                    let core_mutation = match core_module.get_mutation_mut() {
                        Some(core_mutation) => core_mutation,
                        None => return Err(HExecError::MutationNotSet(*item_id)),
                    };
                    apply_mattrs_on_change(core_mutation, h_attr_mutations);
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
        match &self.charge {
            TriStateField::Value(charge_type_id) => {
                core_module.set_charge(*charge_type_id);
            }
            TriStateField::None => match core_module.get_charge_mut() {
                Some(core_charge) => core_charge.remove(),
                None => return Err(HExecError::ChargeNotSet(*item_id)),
            },
            TriStateField::Absent => (),
        }
        for proj_def in self.add_projs.iter() {
            core_module
                .add_proj(&proj_def.get_item_id(), proj_def.get_range())
                .map_err(|error| match error {
                    rc::err::AddRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddRangedProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddRangedProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
                })?;
        }
        for proj_def in self.change_projs.iter() {
            core_module
                .get_proj_mut(&proj_def.get_item_id())
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .set_range(proj_def.get_range());
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
        Ok(core_module.into())
    }
}
