use crate::{
    AddMode, AddMutation, AddedItemIdsResp, CmdResps, EffectId, EffectMode, FitId, FitIdBr, ItemId, ItemIdBr,
    ItemTypeId, ModRack, ModuleState, OptionalReload, Spool, ctl::core::shared::EffectModes, err::BrResolveError,
    shared::CmdResidue,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ModuleAddCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    proj_item_ids: Vec<ItemId> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: ModuleAddCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ModuleAddCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    proj_item_ids: Vec<ItemIdBr> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: ModuleAddCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
struct ModuleAddCmdShared {
    rack: ModRack,
    add_mode: AddMode,
    type_id: ItemTypeId,
    state: ModuleState,
    mutation: Option<AddMutation> = None,
    charge_type_id: Option<ItemTypeId> = None,
    spool: Option<Spool> = None,
    optional_reload: Option<OptionalReload> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ModuleAddCmdCtxFit {
    fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ModuleAddCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ModuleAddCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ModuleAddCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ModuleAddCmd {
    pub fn new(rack: ModRack, add_mode: AddMode, type_id: ItemTypeId, state: ModuleState) -> Self {
        Self {
            shared: ModuleAddCmdShared {
                rack,
                add_mode,
                type_id,
                state,
                ..
            },
            ..
        }
    }
    pub fn with_mutation(mut self, mutation: AddMutation) -> Self {
        self.shared.mutation = Some(mutation);
        self
    }
    pub fn with_charge_type_id(mut self, charge_type_id: ItemTypeId) -> Self {
        self.shared.charge_type_id = Some(charge_type_id);
        self
    }
    pub fn with_spool(mut self, spool: Spool) -> Self {
        self.shared.spool = Some(spool);
        self
    }
    pub fn with_optional_reload(mut self, optional_reload: OptionalReload) -> Self {
        self.shared.optional_reload = Some(optional_reload);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.shared.effect_modes.extend(effect_modes);
        self
    }
}

impl ModuleAddCmdBr {
    pub fn new(rack: ModRack, add_mode: AddMode, type_id: ItemTypeId, state: ModuleState) -> Self {
        Self {
            shared: ModuleAddCmdShared {
                rack,
                add_mode,
                type_id,
                state,
                ..
            },
            ..
        }
    }
    pub fn with_mutation(mut self, mutation: AddMutation) -> Self {
        self.shared.mutation = Some(mutation);
        self
    }
    pub fn with_charge_type_id(mut self, charge_type_id: ItemTypeId) -> Self {
        self.shared.charge_type_id = Some(charge_type_id);
        self
    }
    pub fn with_spool(mut self, spool: Spool) -> Self {
        self.shared.spool = Some(spool);
        self
    }
    pub fn with_optional_reload(mut self, optional_reload: OptionalReload) -> Self {
        self.shared.optional_reload = Some(optional_reload);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.shared.effect_modes.extend(effect_modes);
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
            shared: self.shared,
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
impl ModuleAddCmdBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self.proj_item_ids.is_empty() {
            true => CmdResidue::MutInfallible,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}
impl ModuleAddCmdCtxFitBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self.core.proj_item_ids.is_empty() {
            true => CmdResidue::MutFallibleClean,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}

impl ModuleAddCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, ModuleAddError> {
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
