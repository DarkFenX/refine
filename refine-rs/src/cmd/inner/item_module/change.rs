use rc::ItemCommon;

use crate::{
    cmd::shared::{BackrefRenderError, ChangeMutation, ChangedItemIdsResp, CmdResps, EffectModes, ItemIdBackref},
    util::TriStateField,
};

// Commands with full context
pub(in crate::cmd) struct ICmdModuleChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdModuleChangeICtxBIds = ICmdModuleChangeICtxBIds { .. },
}
pub(crate) struct ICmdModuleChangeFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: ICmdModuleChangeICtxRIds,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdModuleChangeICtxBIds {
    pub(in crate::cmd) shared: ICmdModuleChangeShared = ICmdModuleChangeShared { .. },
    pub(in crate::cmd) add_proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
    pub(in crate::cmd) rm_proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
}
pub(in crate::cmd) struct ICmdModuleChangeICtxRIds {
    pub(in crate::cmd) shared: ICmdModuleChangeShared = ICmdModuleChangeShared { .. },
    pub(in crate::cmd) add_proj_item_ids: Vec<rc::ItemId> = Vec::new(),
    pub(in crate::cmd) rm_proj_item_ids: Vec<rc::ItemId> = Vec::new(),
}
pub(in crate::cmd) struct ICmdModuleChangeShared {
    pub(in crate::cmd) type_id: Option<rc::ItemTypeId> = None,
    pub(in crate::cmd) move_: Option<rc::MoveMode> = None,
    pub(in crate::cmd) state: Option<rc::ModuleState> = None,
    pub(in crate::cmd) mutation: TriStateField<ChangeMutation> = TriStateField::Absent,
    pub(in crate::cmd) charge_type_id: TriStateField<rc::ItemTypeId> = TriStateField::Absent,
    pub(in crate::cmd) spool: TriStateField<rc::Spool> = TriStateField::Absent,
    pub(in crate::cmd) optional_reload: TriStateField<rc::OptionalReload> = TriStateField::Absent,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdModuleChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdModuleChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdModuleChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdModuleChangeICtxBIds {
    fn render(self, resps: &CmdResps) -> Result<ICmdModuleChangeICtxRIds, BackrefRenderError> {
        Ok(ICmdModuleChangeICtxRIds {
            shared: self.shared,
            add_proj_item_ids: resps.render_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.render_item_ids(self.rm_proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdModuleChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeModuleError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeModuleError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ItemChangeModuleError),
}

impl ICmdModuleChangeICtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeModuleError> {
        let mut resp = ChangedItemIdsResp::default();
        let core_module = core_item.dc_module()?;
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_module.get_proj_mut(projectee_item_id)?.remove();
        }
        if let Some(type_id) = self.shared.type_id {
            core_module.set_type_id(type_id);
        }
        if let Some(move_) = self.shared.move_ {
            core_module.move_(move_);
        }
        if let Some(state) = self.shared.state {
            core_module.set_state(state);
        }
        match &self.shared.mutation {
            TriStateField::Value(mutation) => {
                // Mutates item or updates existing mutation
                if let Some(mutator_id) = mutation.mutator_id {
                    match core_module.get_mutation_mut() {
                        Some(core_mutation) => core_mutation.set_mutator_type_id(mutator_id),
                        None => core_module.mutate(mutator_id).unwrap(),
                    };
                }
                if !mutation.attrs.is_empty() {
                    match core_module.get_mutation_mut() {
                        Some(mut core_mutation) => mutation.apply_attrs(&mut core_mutation),
                        None => return Err(ItemChangeModuleError::NotMutated(core_module.get_item_id())),
                    };
                }
            }
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
                let core_charge = core_module.set_charge_type_id(charge_type_id);
                // Set response charge ID only if we actually did it
                resp = ChangedItemIdsResp::from_core_charge(core_charge);
            }
            TriStateField::None => {
                // Do nothing if module had no charge
                if let Some(core_charge) = core_module.get_charge_mut() {
                    core_charge.remove()
                }
            }
            TriStateField::Absent => (),
        }
        match self.shared.spool {
            TriStateField::Value(spool) => core_module.set_spool(Some(spool)),
            TriStateField::None => core_module.set_spool(None),
            TriStateField::Absent => (),
        }
        match self.shared.optional_reload {
            TriStateField::Value(optional_reload) => core_module.set_optional_reload(Some(optional_reload)),
            TriStateField::None => core_module.set_optional_reload(None),
            TriStateField::Absent => (),
        }
        self.shared.effect_modes.apply(core_module);
        for projectee_item_id in self.add_proj_item_ids.iter() {
            core_module.add_proj(projectee_item_id)?;
        }
        Ok(resp)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeModuleError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
    #[error("unable to mutate attributes: item {0} is not mutated")]
    NotMutated(rc::ItemId),
    #[error("unable to add projection: {0}")]
    ProjAddFailed(#[from] rc::err::AddProjError),
    #[error("unable to remove projection: {0}")]
    ProjRemoveFailed(#[from] rc::err::GetRangedProjError),
}
