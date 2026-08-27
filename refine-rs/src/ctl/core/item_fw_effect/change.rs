use crate::{
    ChangedItemIdsResp, CmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId, ctl::core::shared::EffectModes,
    err::BrResolveError, shared::CmdResidue,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FwEffectChangeCmd {
    type_id: Option<ItemTypeId>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
pub type FwEffectChangeCmdCtxItem = FwEffectChangeCmdCtxItemGen<ItemId>;
pub type FwEffectChangeCmdCtxItemBr = FwEffectChangeCmdCtxItemGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FwEffectChangeCmdCtxItemGen<I> {
    item_id: I,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FwEffectChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FwEffectChangeCmd {
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FwEffectChangeCmd {
    pub(in crate::ctl) fn into_ctx_item(self, item_id: ItemId) -> FwEffectChangeCmdCtxItem {
        FwEffectChangeCmdCtxItem { item_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> FwEffectChangeCmdCtxItemBr {
        FwEffectChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FwEffectChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<FwEffectChangeCmdCtxItem, BrResolveError> {
        Ok(FwEffectChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FwEffectChangeCmd {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        // Assume the command always mutates (even if it does not with none of fields set)
        CmdResidue::MutFallibleClean
    }
}
impl<I> FwEffectChangeCmdCtxItemGen<I> {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}

impl FwEffectChangeCmd {
    pub(in crate::ctl) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, FwEffectChangeError> {
        let core_fw_effect = core_item.dc_fw_effect()?;
        if let Some(type_id) = self.type_id {
            core_fw_effect.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_fw_effect.set_state(state);
        }
        self.effect_modes.apply(core_fw_effect);
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FwEffectChangeError {
    #[error(transparent)]
    ItemIsNotFwEffect(#[from] rc::err::ItemKindMatchError),
}

impl FwEffectChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetFwEffectChangeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetFwEffectChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::ItemGetError),
    #[error(transparent)]
    ItemIsNotFwEffect(rc::err::ItemKindMatchError),
}
impl From<FwEffectChangeError> for ItemGetFwEffectChangeError {
    fn from(err: FwEffectChangeError) -> Self {
        match err {
            FwEffectChangeError::ItemIsNotFwEffect(inner) => Self::ItemIsNotFwEffect(inner),
        }
    }
}
