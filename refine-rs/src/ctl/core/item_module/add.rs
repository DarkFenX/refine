use crate::{
    AddMode, AddMutation, AddedItemIdsResp, CmdResps, EffectId, EffectMode, FitId, FitIdBr, ItemId, ItemIdBr,
    ItemTypeId, ModRack, ModuleState, OptionalReload, Spool, ctl::core::shared::EffectModes, err::BrResolveError,
    shared::CmdResidue,
};

// Core commands
pub type ModuleAddCmd = ModuleAddCmdGen<ItemId>;
pub type ModuleAddCmdBr = ModuleAddCmdGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ModuleAddCmdGen<I> {
    rack: ModRack,
    add_mode: AddMode,
    type_id: ItemTypeId,
    state: ModuleState,
    mutation: Option<AddMutation> = None,
    charge_type_id: Option<ItemTypeId> = None,
    spool: Option<Spool> = None,
    optional_reload: Option<OptionalReload> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    proj_item_ids: Vec<I> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
pub type ModuleAddCmdCtxFit = ModuleAddCmdCtxFitGen<FitId, ItemId>;
pub type ModuleAddCmdCtxFitBr = ModuleAddCmdCtxFitGen<FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ModuleAddCmdCtxFitGen<F, I> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ModuleAddCmdGen<I>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> ModuleAddCmdGen<I> {
    pub fn new(rack: ModRack, add_mode: AddMode, type_id: ItemTypeId, state: ModuleState) -> Self {
        Self {
            rack,
            add_mode,
            type_id,
            state,
            ..
        }
    }
    pub fn with_mutation(mut self, mutation: AddMutation) -> Self {
        self.mutation = Some(mutation);
        self
    }
    pub fn with_charge_type_id(mut self, charge_type_id: ItemTypeId) -> Self {
        self.charge_type_id = Some(charge_type_id);
        self
    }
    pub fn with_spool(mut self, spool: Spool) -> Self {
        self.spool = Some(spool);
        self
    }
    pub fn with_optional_reload(mut self, optional_reload: OptionalReload) -> Self {
        self.optional_reload = Some(optional_reload);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl IntoIterator<Item = I>) -> Self {
        self.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl IntoIterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ModuleAddCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> ModuleAddCmdCtxFit {
        ModuleAddCmdCtxFit { fit_id, core: self }
    }
}
impl ModuleAddCmdBr {
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> ModuleAddCmdCtxFitBr {
        ModuleAddCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ModuleAddCmdBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ModuleAddCmd, BrResolveError> {
        Ok(ModuleAddCmd {
            proj_item_ids: resps.resolve_item_ids(self.proj_item_ids)?,
            rack: self.rack,
            add_mode: self.add_mode,
            type_id: self.type_id,
            state: self.state,
            mutation: self.mutation,
            charge_type_id: self.charge_type_id,
            spool: self.spool,
            optional_reload: self.optional_reload,
            effect_modes: self.effect_modes,
        })
    }
}

impl ModuleAddCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ModuleAddCmdCtxFit, BrResolveError> {
        Ok(ModuleAddCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> ModuleAddCmdGen<I> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        match self.proj_item_ids.is_empty() {
            true => CmdResidue::MutInfallible,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}
impl<F, I> ModuleAddCmdCtxFitGen<F, I> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        match self.core.proj_item_ids.is_empty() {
            true => CmdResidue::MutFallibleClean,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}

impl ModuleAddCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, ModuleAddError> {
        let mut core_module = core_fit.add_module(self.rack, self.add_mode, self.type_id, self.state);
        if let Some(mutation) = self.mutation.as_ref() {
            let mut core_mutation = core_module.mutate(mutation.mutator_id).unwrap();
            mutation.apply_attrs(&mut core_mutation);
        }
        if let Some(charge_type_id) = self.charge_type_id {
            core_module.set_charge_type_id(charge_type_id);
        }
        if let Some(spool) = self.spool {
            core_module.set_spool(Some(spool));
        }
        if let Some(optional_reload) = self.optional_reload {
            core_module.set_optional_reload(Some(optional_reload));
        }
        self.effect_modes.apply(&mut core_module);
        for projectee_item_id in self.proj_item_ids.iter() {
            core_module.add_proj(projectee_item_id)?;
        }
        Ok(AddedItemIdsResp::from_core_module(core_module))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ModuleAddError {
    #[error("failed to add projection")]
    ProjAdd(#[from] rc::err::ProjAddError),
}

impl ModuleAddCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, FitGetModuleAddError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetModuleAddError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
    #[error("failed to add projection")]
    ProjAdd(#[source] rc::err::ProjAddError),
}
impl From<ModuleAddError> for FitGetModuleAddError {
    fn from(err: ModuleAddError) -> Self {
        match err {
            ModuleAddError::ProjAdd(inner) => Self::ProjAdd(inner),
        }
    }
}
