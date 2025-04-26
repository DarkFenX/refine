use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, HMutationOnChange, HProjDef, HProjDefFull, apply_effect_modes},
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
                    // Remove old mutation if we had any, ignore any errors on the way
                    let _ = core_module.unmutate();
                    let mutation = rc::ItemAddMutation::new(*mutator_id);
                    core_module.mutate(mutation).unwrap();
                }
                HMutationOnChange::AddFull(mutation) => {
                    // Remove old mutation if we had any, ignore any errors on the way
                    let _ = core_module.unmutate();
                    core_module.mutate(mutation.into()).unwrap();
                }
                HMutationOnChange::ChangeAttrs(attr_mutations) => {
                    let attr_mutations = attr_mutations
                        .iter()
                        .map(|(k, v)| rc::ItemChangeAttrMutation::new(*k, v.as_ref().map(|v| v.into())))
                        .collect();
                    core_module
                        .change_mutation(attr_mutations)
                        .map_err(|error| match error {
                            rc::err::ChangeModuleMutationError::MutationNotSet(e) => HExecError::MutationNotSet(e),
                        })?;
                }
            },
            TriStateField::None => {
                // Do nothing if mutation was not there
                let _ = core_module.unmutate();
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
                    rc::err::AddModuleProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddModuleProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddModuleProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
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
