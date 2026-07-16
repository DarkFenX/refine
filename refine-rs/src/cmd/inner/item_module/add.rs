use crate::{
    AddMutation, AddedItemIdsResp, CmdResps, FitIdBackref, ItemIdBackref, cmd::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context
pub(in crate::cmd) struct ICmdModuleAddFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdModuleAddICtxBIds,
}
pub(crate) struct ICmdModuleAddFCtxRIds {
    pub(in crate::cmd) fit_id: rc::FitId,
    pub(in crate::cmd) ictx_cmd: ICmdModuleAddICtxRIds,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdModuleAddICtxBIds {
    pub(in crate::cmd) shared: ICmdModuleAddShared,
    pub(in crate::cmd) proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
}
pub(crate) struct ICmdModuleAddICtxRIds {
    pub(in crate::cmd) shared: ICmdModuleAddShared,
    pub(in crate::cmd) proj_item_ids: Vec<rc::ItemId> = Vec::new(),
}
pub(in crate::cmd) struct ICmdModuleAddShared {
    pub(in crate::cmd) rack: rc::ModRack,
    pub(in crate::cmd) add_mode: rc::AddMode,
    pub(in crate::cmd) type_id: rc::ItemTypeId,
    pub(in crate::cmd) state: rc::ModuleState,
    pub(in crate::cmd) mutation: Option<AddMutation> = None,
    pub(in crate::cmd) charge_type_id: Option<rc::ItemTypeId> = None,
    pub(in crate::cmd) spool: Option<rc::Spool> = None,
    pub(in crate::cmd) optional_reload: Option<rc::OptionalReload> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdModuleAddFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdModuleAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdModuleAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdModuleAddICtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdModuleAddICtxRIds, BackrefRenderError> {
        Ok(ICmdModuleAddICtxRIds {
            shared: self.shared,
            proj_item_ids: resps.render_item_ids(self.proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdModuleAddFCtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddModuleError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddModuleError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
    #[error("{0}")]
    AddFailed(#[from] FitAddModuleError),
}

impl ICmdModuleAddICtxRIds {
    pub(in crate::cmd) fn execute(&self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, FitAddModuleError> {
        let mut core_module = core_fit.add_module(
            self.shared.rack,
            self.shared.add_mode,
            self.shared.type_id,
            self.shared.state,
        );
        if let Some(mutation) = self.shared.mutation.as_ref() {
            let mut core_mutation = core_module.mutate(mutation.mutator_id).unwrap();
            mutation.apply_attrs(&mut core_mutation);
        }
        if let Some(charge_type_id) = self.shared.charge_type_id {
            core_module.set_charge_type_id(charge_type_id);
        }
        if let Some(spool) = self.shared.spool {
            core_module.set_spool(Some(spool));
        }
        if let Some(optional_reload) = self.shared.optional_reload {
            core_module.set_optional_reload(Some(optional_reload));
        }
        self.shared.effect_modes.apply(&mut core_module);
        for projectee_item_id in self.proj_item_ids.iter() {
            core_module.add_proj(projectee_item_id)?;
        }
        Ok(AddedItemIdsResp::from_core_module(core_module))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitAddModuleError {
    #[error("failed to add projection: {0}")]
    ProjAddFailed(#[from] rc::err::AddProjError),
}
