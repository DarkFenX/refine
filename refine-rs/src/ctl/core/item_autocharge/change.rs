use crate::{
    CmdResps, EffectId, EffectMode, ItemId, ItemIdBr,
    ctl::{ChangedItemIdsResp, core::shared::EffectModes},
    err::BrResolveError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct AutochargeChangeCmd {
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}
const impl Default for AutochargeChangeCmd {
    fn default() -> Self {
        Self {
            state: Default::default(),
            effect_modes: Default::default(),
        }
    }
}

// Extra context commands
pub type AutochargeChangeCmdCtxItem = AutochargeChangeCmdCtxItemGen<ItemId>;
pub type AutochargeChangeCmdCtxItemBr = AutochargeChangeCmdCtxItemGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct AutochargeChangeCmdCtxItemGen<I> {
    item_id: I,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: AutochargeChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AutochargeChangeCmd {
    pub const fn new() -> Self {
        Self::default()
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
impl AutochargeChangeCmd {
    pub(in crate::ctl) const fn into_ctx_item(self, item_id: ItemId) -> AutochargeChangeCmdCtxItem {
        AutochargeChangeCmdCtxItem { item_id, core: self }
    }
    pub(in crate::ctl) const fn into_ctx_item_br(
        self,
        item_id: impl [const] Into<ItemIdBr>,
    ) -> AutochargeChangeCmdCtxItemBr {
        AutochargeChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AutochargeChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<AutochargeChangeCmdCtxItem, BrResolveError> {
        Ok(AutochargeChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AutochargeChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::Item) -> Result<ChangedItemIdsResp, AutochargeChangeError> {
        let Self { state, effect_modes } = self;
        let core_autocharge = core_item.dc_autocharge()?;
        if let Some(state) = state {
            core_autocharge.set_state(state);
        }
        effect_modes.apply(core_autocharge);
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum AutochargeChangeError {
    #[error(transparent)]
    ItemIsNotAutocharge(#[from] crate::err::core::ItemKindMatchError),
}

impl AutochargeChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::Sol,
    ) -> Result<ChangedItemIdsResp, ItemGetAutochargeChangeError> {
        let mut core_item = core_sol
            .get_item(&self.item_id)
            .map_err(ItemGetAutochargeChangeError::ItemGet)?;
        Ok(self.core.execute(&mut core_item)?)
    }
    pub(in crate::ctl) fn execute_via_fit(
        self,
        core_fit: &mut rc::Fit,
    ) -> Result<ChangedItemIdsResp, FitItemGetAutochargeChangeError> {
        let mut core_item = core_fit
            .get_item(&self.item_id)
            .map_err(FitItemGetAutochargeChangeError::ItemGet)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}

pub type ItemGetAutochargeChangeError = GenItemGetAutochargeChangeError<crate::err::core::ItemGetError>;
pub type FitItemGetAutochargeChangeError = GenItemGetAutochargeChangeError<crate::err::core::FitItemGetError>;

#[derive(thiserror::Error, Debug)]
pub enum GenItemGetAutochargeChangeError<E> {
    #[error(transparent)]
    ItemGet(E),
    #[error(transparent)]
    ItemIsNotAutocharge(crate::err::core::ItemKindMatchError),
}
impl<E> From<AutochargeChangeError> for GenItemGetAutochargeChangeError<E> {
    fn from(err: AutochargeChangeError) -> Self {
        match err {
            AutochargeChangeError::ItemIsNotAutocharge(inner) => Self::ItemIsNotAutocharge(inner),
        }
    }
}
