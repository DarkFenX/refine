use crate::{
    ChangedItemIdsResp, CmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId, ctl::core::shared::EffectModes,
    err::BrResolveError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ProjEffectChangeCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    add_proj_item_ids: Vec<ItemId>,
    #[cfg_attr(feature = "serde", serde(default))]
    rm_proj_item_ids: Vec<ItemId>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: ProjEffectChangeCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ProjEffectChangeCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    add_proj_item_ids: Vec<ItemIdBr>,
    #[cfg_attr(feature = "serde", serde(default))]
    rm_proj_item_ids: Vec<ItemIdBr>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: ProjEffectChangeCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
struct ProjEffectChangeCmdShared {
    type_id: Option<ItemTypeId>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ProjEffectChangeCmdCtxItem {
    item_id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ProjEffectChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ProjEffectChangeCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ProjEffectChangeCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ProjEffectChangeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.shared.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.shared.state = Some(state);
        self
    }
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.rm_proj_item_ids.extend(rm_proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.shared.effect_modes.extend(effect_modes);
        self
    }
}

impl ProjEffectChangeCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.shared.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.shared.state = Some(state);
        self
    }
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.rm_proj_item_ids.extend(rm_proj_item_ids);
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
impl ProjEffectChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ProjEffectChangeCmdCtxItem, BrResolveError> {
        Ok(ProjEffectChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

impl ProjEffectChangeCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<ProjEffectChangeCmd, BrResolveError> {
        Ok(ProjEffectChangeCmd {
            add_proj_item_ids: resps.resolve_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.resolve_item_ids(self.rm_proj_item_ids)?,
            shared: self.shared,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ProjEffectChangeCmd {
    pub(in crate::ctl) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ProjEffectChangeError> {
        let core_proj_effect = core_item.dc_proj_effect()?;
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_proj_effect.get_proj_mut(projectee_item_id)?.remove();
        }
        if let Some(type_id) = self.shared.type_id {
            core_proj_effect.set_type_id(type_id);
        }
        if let Some(state) = self.shared.state {
            core_proj_effect.set_state(state);
        }
        self.shared.effect_modes.apply(core_proj_effect);
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
