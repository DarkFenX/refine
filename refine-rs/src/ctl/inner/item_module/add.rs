use crate::{
    AddMode, AddMutation, AddedItemIdsResp, CtlCmdResps, FitId, FitIdBackref, ItemId, ItemIdBackref, ItemTypeId,
    ModRack, ModuleState, OptionalReload, Spool, ctl::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdModuleAddFCtxBIds {
    pub(in crate::ctl) fit_id: FitIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdModuleAddICtxBIds,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdModuleAddFCtxRIds {
    pub(in crate::ctl) fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdModuleAddICtxRIds,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdModuleAddICtxBIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) shared: ICmdModuleAddShared,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdModuleAddICtxRIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) shared: ICmdModuleAddShared,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) proj_item_ids: Vec<ItemId> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdModuleAddShared {
    pub(in crate::ctl) rack: ModRack,
    pub(in crate::ctl) add_mode: AddMode,
    pub(in crate::ctl) type_id: ItemTypeId,
    pub(in crate::ctl) state: ModuleState,
    pub(in crate::ctl) mutation: Option<AddMutation> = None,
    pub(in crate::ctl) charge_type_id: Option<ItemTypeId> = None,
    pub(in crate::ctl) spool: Option<Spool> = None,
    pub(in crate::ctl) optional_reload: Option<OptionalReload> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdModuleAddFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdModuleAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdModuleAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdModuleAddICtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdModuleAddICtxRIds, BackrefRenderError> {
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
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddModuleError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddModuleError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
    #[error("failed to add projection")]
    ProjAdd(#[source] rc::err::AddProjError),
}
impl From<FitAddModuleError> for GetFitAddModuleError {
    fn from(err: FitAddModuleError) -> Self {
        match err {
            FitAddModuleError::ProjAdd(inner) => Self::ProjAdd(inner),
        }
    }
}

impl ICmdModuleAddICtxRIds {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, FitAddModuleError> {
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
    #[error("failed to add projection")]
    ProjAdd(#[from] rc::err::AddProjError),
}
