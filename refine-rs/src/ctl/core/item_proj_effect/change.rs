use crate::{
    ChangedItemIdsResp, CmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId, ctl::core::shared::EffectModes,
    err::BrResolveError, shared::CmdResidue,
};

// Core commands
pub type ProjEffectChangeCmd = ProjEffectChangeCmdGen<ItemId>;
pub type ProjEffectChangeCmdBr = ProjEffectChangeCmdGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ProjEffectChangeCmdGen<I> {
    type_id: Option<ItemTypeId>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    add_proj_item_ids: Vec<I>,
    #[cfg_attr(feature = "serde", serde(default))]
    rm_proj_item_ids: Vec<I>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}
impl<I> Default for ProjEffectChangeCmdGen<I> {
    fn default() -> Self {
        Self {
            type_id: Default::default(),
            state: Default::default(),
            add_proj_item_ids: Default::default(),
            rm_proj_item_ids: Default::default(),
            effect_modes: Default::default(),
        }
    }
}

// Extra context commands
pub type ProjEffectChangeCmdCtxItem = ProjEffectChangeCmdCtxItemGen<ItemId>;
pub type ProjEffectChangeCmdCtxItemBr = ProjEffectChangeCmdCtxItemGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ProjEffectChangeCmdCtxItemGen<I> {
    item_id: I,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ProjEffectChangeCmdGen<I>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> ProjEffectChangeCmdGen<I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.state = Some(state);
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
impl ProjEffectChangeCmd {
    pub(in crate::ctl) fn into_ctx_item(self, item_id: ItemId) -> ProjEffectChangeCmdCtxItem {
        ProjEffectChangeCmdCtxItem { item_id, core: self }
    }
}
impl ProjEffectChangeCmdBr {
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> ProjEffectChangeCmdCtxItemBr {
        ProjEffectChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ProjEffectChangeCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<ProjEffectChangeCmd, BrResolveError> {
        Ok(ProjEffectChangeCmd {
            add_proj_item_ids: resps.resolve_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.resolve_item_ids(self.rm_proj_item_ids)?,
            type_id: self.type_id,
            state: self.state,
            effect_modes: self.effect_modes,
        })
    }
}

impl ProjEffectChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ProjEffectChangeCmdCtxItem, BrResolveError> {
        Ok(ProjEffectChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> ProjEffectChangeCmdGen<I> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        // Assume the command always mutates (even if it does not with none of fields set)
        if !self.rm_proj_item_ids.is_empty() || !self.add_proj_item_ids.is_empty() {
            return CmdResidue::MutFallibleDirty;
        }
        CmdResidue::MutFallibleClean
    }
}
impl<I> ProjEffectChangeCmdCtxItemGen<I> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        self.core.exec_residue()
    }
}

impl ProjEffectChangeCmd {
    pub(in crate::ctl) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ProjEffectChangeError> {
        let core_proj_effect = core_item.dc_proj_effect()?;
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_proj_effect.get_proj_mut(projectee_item_id)?.remove();
        }
        if let Some(type_id) = self.type_id {
            core_proj_effect.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_proj_effect.set_state(state);
        }
        self.effect_modes.apply(core_proj_effect);
        for projectee_item_id in self.add_proj_item_ids.iter() {
            core_proj_effect.add_proj(projectee_item_id)?;
        }
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ProjEffectChangeError {
    #[error(transparent)]
    ItemIsNotProjEffect(#[from] rc::err::ItemKindMatchError),
    #[error("unable to add projection")]
    ProjAdd(#[from] rc::err::ProjAddError),
    #[error("unable to remove projection")]
    ProjRemove(#[from] rc::err::ProjGetError),
}

impl ProjEffectChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetProjEffectChangeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetProjEffectChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::ItemGetError),
    #[error(transparent)]
    ItemIsNotProjEffect(rc::err::ItemKindMatchError),
    #[error("unable to add projection")]
    ProjAdd(#[source] rc::err::ProjAddError),
    #[error("unable to remove projection")]
    ProjRemove(#[source] rc::err::ProjGetError),
}
impl From<ProjEffectChangeError> for ItemGetProjEffectChangeError {
    fn from(err: ProjEffectChangeError) -> Self {
        match err {
            ProjEffectChangeError::ItemIsNotProjEffect(inner) => Self::ItemIsNotProjEffect(inner),
            ProjEffectChangeError::ProjAdd(inner) => Self::ProjAdd(inner),
            ProjEffectChangeError::ProjRemove(inner) => Self::ProjRemove(inner),
        }
    }
}
