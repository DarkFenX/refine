use rc::ItemCommon;

use crate::{
    ChangeMutation, ChangedItemIdsResp, CmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId, ModuleState,
    MoveMode, OptionalReload, Spool, TriStateField, ctl::core::shared::EffectModes, err::BrResolveError,
    shared::CmdResidue,
};

// Core commands
pub type ModuleChangeCmd = ModuleChangeCmdGen<ItemId>;
pub type ModuleChangeCmdBr = ModuleChangeCmdGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ModuleChangeCmdGen<I> {
    type_id: Option<ItemTypeId>,
    #[cfg_attr(feature = "serde", serde(rename = "move"))]
    move_: Option<MoveMode>,
    state: Option<ModuleState>,
    #[cfg_attr(feature = "serde", serde(default))]
    mutation: TriStateField<ChangeMutation>,
    #[cfg_attr(feature = "serde", serde(default))]
    charge_type_id: TriStateField<ItemTypeId>,
    #[cfg_attr(feature = "serde", serde(default))]
    spool: TriStateField<Spool>,
    #[cfg_attr(feature = "serde", serde(default))]
    optional_reload: TriStateField<OptionalReload>,
    #[cfg_attr(feature = "serde", serde(default))]
    add_proj_item_ids: Vec<I>,
    #[cfg_attr(feature = "serde", serde(default))]
    rm_proj_item_ids: Vec<I>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}
impl<I> Default for ModuleChangeCmdGen<I> {
    fn default() -> Self {
        Self {
            type_id: Default::default(),
            move_: Default::default(),
            state: Default::default(),
            mutation: Default::default(),
            charge_type_id: Default::default(),
            spool: Default::default(),
            optional_reload: Default::default(),
            add_proj_item_ids: Default::default(),
            rm_proj_item_ids: Default::default(),
            effect_modes: Default::default(),
        }
    }
}

// Extra context commands
pub type ModuleChangeCmdCtxItem = ModuleChangeCmdCtxItemGen<ItemId>;
pub type ModuleChangeCmdCtxItemBr = ModuleChangeCmdCtxItemGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ModuleChangeCmdCtxItemGen<I> {
    item_id: I,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ModuleChangeCmdGen<I>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> ModuleChangeCmdGen<I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.type_id = Some(type_id);
        self
    }
    pub fn with_move(mut self, move_: MoveMode) -> Self {
        self.move_ = Some(move_);
        self
    }
    pub fn with_state(mut self, state: ModuleState) -> Self {
        self.state = Some(state);
        self
    }
    pub fn with_mutation(mut self, mutation: Option<ChangeMutation>) -> Self {
        self.mutation = mutation.into();
        self
    }
    pub fn with_charge_type_id(mut self, charge_type_id: Option<ItemTypeId>) -> Self {
        self.charge_type_id = charge_type_id.into();
        self
    }
    pub fn with_spool(mut self, spool: Option<Spool>) -> Self {
        self.spool = spool.into();
        self
    }
    pub fn with_optional_reload(mut self, optional_reload: Option<OptionalReload>) -> Self {
        self.optional_reload = optional_reload.into();
        self
    }
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl Iterator<Item = I>) -> Self {
        self.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl Iterator<Item = I>) -> Self {
        self.rm_proj_item_ids.extend(rm_proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ModuleChangeCmd {
    pub(in crate::ctl) fn into_ctx_item(self, item_id: ItemId) -> ModuleChangeCmdCtxItem {
        ModuleChangeCmdCtxItem { item_id, core: self }
    }
}
impl ModuleChangeCmdBr {
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> ModuleChangeCmdCtxItemBr {
        ModuleChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ModuleChangeCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<ModuleChangeCmd, BrResolveError> {
        Ok(ModuleChangeCmd {
            add_proj_item_ids: resps.resolve_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.resolve_item_ids(self.rm_proj_item_ids)?,
            type_id: self.type_id,
            move_: self.move_,
            state: self.state,
            mutation: self.mutation,
            charge_type_id: self.charge_type_id,
            spool: self.spool,
            optional_reload: self.optional_reload,
            effect_modes: self.effect_modes,
        })
    }
}

impl ModuleChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ModuleChangeCmdCtxItem, BrResolveError> {
        Ok(ModuleChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> ModuleChangeCmdGen<I> {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        // Assume the command always mutates (even if it does not with none of fields set)
        if !self.rm_proj_item_ids.is_empty() || !self.add_proj_item_ids.is_empty() {
            return CmdResidue::MutFallibleDirty;
        }
        if let TriStateField::Value(mutation) = &self.mutation
            && !mutation.attrs.is_empty()
        {
            return CmdResidue::MutFallibleDirty;
        }
        CmdResidue::MutFallibleClean
    }
}
impl<I> ModuleChangeCmdCtxItemGen<I> {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        self.core.exec_residue()
    }
}

impl ModuleChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, ModuleChangeError> {
        let mut resp = ChangedItemIdsResp::default();
        let core_module = core_item.dc_module()?;
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_module.get_proj_mut(projectee_item_id)?.remove();
        }
        if let Some(type_id) = self.type_id {
            core_module.set_type_id(type_id);
        }
        if let Some(move_) = self.move_ {
            core_module.move_(move_);
        }
        if let Some(state) = self.state {
            core_module.set_state(state);
        }
        match &self.mutation {
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
                        None => return Err(ModuleChangeError::NotMutated(core_module.get_item_id())),
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
        match self.charge_type_id {
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
        match self.spool {
            TriStateField::Value(spool) => core_module.set_spool(Some(spool)),
            TriStateField::None => core_module.set_spool(None),
            TriStateField::Absent => (),
        }
        match self.optional_reload {
            TriStateField::Value(optional_reload) => core_module.set_optional_reload(Some(optional_reload)),
            TriStateField::None => core_module.set_optional_reload(None),
            TriStateField::Absent => (),
        }
        self.effect_modes.apply(core_module);
        for projectee_item_id in self.add_proj_item_ids.iter() {
            core_module.add_proj(projectee_item_id)?;
        }
        Ok(resp)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ModuleChangeError {
    #[error(transparent)]
    ItemIsNotModule(#[from] rc::err::ItemKindMatchError),
    #[error("unable to mutate attributes: item {0} is not mutated")]
    NotMutated(ItemId),
    #[error("unable to add projection")]
    ProjAdd(#[from] rc::err::ProjAddError),
    #[error("unable to remove projection")]
    ProjRemove(#[from] rc::err::ProjGetError),
}

impl ModuleChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetModuleChangeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetModuleChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::ItemGetError),
    #[error(transparent)]
    ItemIsNotModule(rc::err::ItemKindMatchError),
    #[error("unable to mutate attributes: item {0} is not mutated")]
    NotMutated(ItemId),
    #[error("unable to add projection")]
    ProjAdd(#[source] rc::err::ProjAddError),
    #[error("unable to remove projection")]
    ProjRemove(#[source] rc::err::ProjGetError),
}
impl From<ModuleChangeError> for ItemGetModuleChangeError {
    fn from(err: ModuleChangeError) -> Self {
        match err {
            ModuleChangeError::ItemIsNotModule(inner) => Self::ItemIsNotModule(inner),
            ModuleChangeError::NotMutated(inner) => Self::NotMutated(inner),
            ModuleChangeError::ProjAdd(inner) => Self::ProjAdd(inner),
            ModuleChangeError::ProjRemove(inner) => Self::ProjRemove(inner),
        }
    }
}
