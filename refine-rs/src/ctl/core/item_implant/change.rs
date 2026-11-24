use crate::{
    CmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId,
    ctl::{ChangedItemIdsResp, core::shared::EffectModes},
    err::BrResolveError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ImplantChangeCmd {
    type_id: Option<ItemTypeId>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}
const impl Default for ImplantChangeCmd {
    fn default() -> Self {
        Self {
            type_id: Default::default(),
            state: Default::default(),
            effect_modes: Default::default(),
        }
    }
}

// Extra context commands
pub type ImplantChangeCmdCtxItem = ImplantChangeCmdCtxItemGen<ItemId>;
pub type ImplantChangeCmdCtxItemBr = ImplantChangeCmdCtxItemGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ImplantChangeCmdCtxItemGen<I> {
    item_id: I,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ImplantChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ImplantChangeCmd {
    pub const fn new() -> Self {
        Self::default()
    }
    pub const fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.type_id = Some(type_id);
        self
    }
    pub const fn with_state(mut self, state: bool) -> Self {
        self.state = Some(state);
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
impl ImplantChangeCmd {
    pub(in crate::ctl) const fn into_ctx_item(self, item_id: ItemId) -> ImplantChangeCmdCtxItem {
        ImplantChangeCmdCtxItem { item_id, core: self }
    }
    pub(in crate::ctl) const fn into_ctx_item_br(
        self,
        item_id: impl [const] Into<ItemIdBr>,
    ) -> ImplantChangeCmdCtxItemBr {
        ImplantChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ImplantChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ImplantChangeCmdCtxItem, BrResolveError> {
        Ok(ImplantChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ImplantChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::Item) -> Result<ChangedItemIdsResp, ImplantChangeError> {
        let Self {
            type_id,
            state,
            effect_modes,
        } = self;
        let core_implant = core_item.dc_implant()?;
        if let Some(type_id) = type_id {
            core_implant.set_type_id(type_id);
        }
        if let Some(state) = state {
            core_implant.set_state(state);
        }
        effect_modes.apply(core_implant);
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ImplantChangeError {
    #[error(transparent)]
    ItemIsNotImplant(#[from] crate::err::core::ItemKindMatchError),
}

impl ImplantChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::Sol,
    ) -> Result<ChangedItemIdsResp, ItemGetImplantChangeError> {
        let mut core_item = core_sol
            .get_item(&self.item_id)
            .map_err(ItemGetImplantChangeError::ItemGet)?;
        Ok(self.core.execute(&mut core_item)?)
    }
    pub(in crate::ctl) fn execute_via_fit(
        self,
        core_fit: &mut rc::Fit,
    ) -> Result<ChangedItemIdsResp, FitItemGetImplantChangeError> {
        let mut core_item = core_fit
            .get_item(&self.item_id)
            .map_err(FitItemGetImplantChangeError::ItemGet)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}

pub type ItemGetImplantChangeError = GenItemGetImplantChangeError<crate::err::core::ItemGetError>;
pub type FitItemGetImplantChangeError = GenItemGetImplantChangeError<crate::err::core::FitItemGetError>;

#[derive(thiserror::Error, Debug)]
pub enum GenItemGetImplantChangeError<E> {
    #[error(transparent)]
    ItemGet(E),
    #[error(transparent)]
    ItemIsNotImplant(crate::err::core::ItemKindMatchError),
}
impl<E> From<ImplantChangeError> for GenItemGetImplantChangeError<E> {
    fn from(err: ImplantChangeError) -> Self {
        match err {
            ImplantChangeError::ItemIsNotImplant(inner) => Self::ItemIsNotImplant(inner),
        }
    }
}
