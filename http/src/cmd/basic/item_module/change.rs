use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::shared::{HChangedItemIdsResp, HCmdResps, HEffectModeMap, HItemIdBackref, HMutationOnChange},
    err::HExecError,
    shared::{HModuleState, HOptionalReload, HSpool},
    util::TriStateField,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HModuleChangeCmdFCtxBIds {
    item_id: HItemIdBackref,
    #[serde(flatten)]
    ictx_cmd: HModuleChangeCmdICtxBIds,
}
pub(crate) struct HModuleChangeCmdFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: HModuleChangeCmdICtxRIds,
}

// Commands with incomplete context
#[derive(Deserialize)]
struct HModuleChangeCmdICtxBIds {
    #[serde(flatten)]
    shared: HModuleChangeCmdShared,
    #[serde(default)]
    add_proj_item_ids: Vec<HItemIdBackref>,
    #[serde(default)]
    rm_proj_item_ids: Vec<HItemIdBackref>,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HModuleChangeCmdICtxRIds {
    #[serde(flatten)]
    shared: HModuleChangeCmdShared,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    add_proj_item_ids: Vec<rc::ItemId>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    rm_proj_item_ids: Vec<rc::ItemId>,
}
#[derive(Deserialize)]
struct HModuleChangeCmdShared {
    type_id: Option<i32>,
    state: Option<HModuleState>,
    #[serde(default)]
    mutation: TriStateField<HMutationOnChange>,
    #[serde(default)]
    charge_type_id: TriStateField<i32>,
    #[serde(default)]
    spool: TriStateField<HSpool>,
    #[serde(default)]
    optional_reload: TriStateField<HOptionalReload>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HModuleChangeCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HModuleChangeCmdFCtxRIds, HExecError> {
        Ok(HModuleChangeCmdFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl HModuleChangeCmdICtxBIds {
    fn render(self, resps: &HCmdResps) -> Result<HModuleChangeCmdICtxRIds, HExecError> {
        Ok(HModuleChangeCmdICtxRIds {
            shared: self.shared,
            add_proj_item_ids: resps.render_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.render_item_ids(self.rm_proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HModuleChangeCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.item_id)
    }
}

impl HModuleChangeCmdICtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HChangedItemIdsResp, HExecError> {
        let mut resp = HChangedItemIdsResp::default();
        let mut core_module = core_sol.get_module_mut(item_id).map_err(|error| match error {
            rc::err::GetModuleError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetModuleError::ItemIsNotModule(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.shared.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_module.set_type_id(core_type_id);
        }
        if let Some(state) = &self.shared.state {
            core_module.set_state(state.into_core());
        }
        match self.shared.mutation.as_ref() {
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
                    let Some(core_mutation) = core_module.get_mutation_mut() else {
                        return Err(HExecError::MutationNotSet(*item_id));
                    };
                    h_attr_mutations.apply(core_mutation);
                }
                // Mutates item, or overwrites mutation, if it was set
                HMutationOnChange::MutatorAndAttrs(h_full_mutation) => {
                    if let Some(core_mutation) = core_module.get_mutation_mut() {
                        core_mutation.remove();
                    }
                    let core_mutator_id = rc::ItemTypeId::from_i32(h_full_mutation.mutator_id);
                    let core_mutation = core_module.mutate(core_mutator_id).unwrap();
                    h_full_mutation.apply_attrs_on_add(core_mutation);
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
        match self.shared.charge_type_id {
            TriStateField::Value(charge_type_id) => {
                let core_charge_type_id = rc::ItemTypeId::from_i32(charge_type_id);
                let core_charge = core_module.set_charge_type_id(core_charge_type_id);
                // Set response charge ID only if we actually did it
                resp = HChangedItemIdsResp::from_core_charge(core_charge);
            }
            TriStateField::None => match core_module.get_charge_mut() {
                Some(core_charge) => core_charge.remove(),
                None => return Err(HExecError::ChargeNotSet(*item_id)),
            },
            TriStateField::Absent => (),
        }
        match self.shared.spool {
            TriStateField::Value(h_spool) => core_module.set_spool(Some(h_spool.into_core())),
            TriStateField::None => core_module.set_spool(None),
            TriStateField::Absent => (),
        }
        match self.shared.optional_reload {
            TriStateField::Value(h_optional_reload) => {
                core_module.set_optional_reload(Some(h_optional_reload.into_core()))
            }
            TriStateField::None => core_module.set_optional_reload(None),
            TriStateField::Absent => (),
        }
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_module
                .get_proj_mut(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .remove();
        }
        for projectee_item_id in self.add_proj_item_ids.iter() {
            core_module.add_proj(projectee_item_id).map_err(|error| match error {
                rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
            })?;
        }
        if let Some(h_effect_modes) = self.shared.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_module);
        }
        Ok(resp)
    }
}
