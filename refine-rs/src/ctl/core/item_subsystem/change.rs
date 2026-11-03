use crate::{
    ChangedItemIdsResp, CmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId, ctl::core::shared::EffectModes,
    err::BrResolveError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SubsystemChangeCmd {
    type_id: Option<ItemTypeId>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
pub type SubsystemChangeCmdCtxItem = SubsystemChangeCmdCtxItemGen<ItemId>;
pub type SubsystemChangeCmdCtxItemBr = SubsystemChangeCmdCtxItemGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct SubsystemChangeCmdCtxItemGen<I> {
    item_id: I,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: SubsystemChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SubsystemChangeCmd {
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
    pub fn with_effect_modes(mut self, effect_modes: impl IntoIterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SubsystemChangeCmd {
    pub(in crate::ctl) fn into_ctx_item(self, item_id: ItemId) -> SubsystemChangeCmdCtxItem {
        SubsystemChangeCmdCtxItem { item_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> SubsystemChangeCmdCtxItemBr {
        SubsystemChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SubsystemChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<SubsystemChangeCmdCtxItem, BrResolveError> {
        Ok(SubsystemChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SubsystemChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::Item) -> Result<ChangedItemIdsResp, SubsystemChangeError> {
        let Self {
            type_id,
            state,
            effect_modes,
        } = self;
        let core_subsystem = core_item.dc_subsystem()?;
        if let Some(type_id) = type_id {
            core_subsystem.set_type_id(type_id);
        }
        if let Some(state) = state {
            core_subsystem.set_state(state);
        }
        effect_modes.apply(core_subsystem);
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum SubsystemChangeError {
    #[error(transparent)]
    ItemIsNotSubsystem(#[from] rc::err::ItemKindMatchError),
}

impl SubsystemChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::Sol,
    ) -> Result<ChangedItemIdsResp, ItemGetSubsystemChangeError> {
        let mut core_item = core_sol
            .get_item(&self.item_id)
            .map_err(ItemGetSubsystemChangeError::ItemGet)?;
        Ok(self.core.execute(&mut core_item)?)
    }
    pub(in crate::ctl) fn execute_via_fit(
        self,
        core_fit: &mut rc::Fit,
    ) -> Result<ChangedItemIdsResp, FitItemGetSubsystemChangeError> {
        let mut core_item = core_fit
            .get_item(&self.item_id)
            .map_err(FitItemGetSubsystemChangeError::ItemGet)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}

pub type ItemGetSubsystemChangeError = GenItemGetSubsystemChangeError<rc::err::ItemGetError>;
pub type FitItemGetSubsystemChangeError = GenItemGetSubsystemChangeError<rc::err::FitItemGetError>;

#[derive(thiserror::Error, Debug)]
pub enum GenItemGetSubsystemChangeError<E> {
    #[error(transparent)]
    ItemGet(E),
    #[error(transparent)]
    ItemIsNotSubsystem(rc::err::ItemKindMatchError),
}
impl<E> From<SubsystemChangeError> for GenItemGetSubsystemChangeError<E> {
    fn from(err: SubsystemChangeError) -> Self {
        match err {
            SubsystemChangeError::ItemIsNotSubsystem(inner) => Self::ItemIsNotSubsystem(inner),
        }
    }
}
