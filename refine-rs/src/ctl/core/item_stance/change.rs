use crate::{
    ChangedItemIdsResp, CmdResps, EffectId, EffectMode, FitId, FitIdBr, ItemId, ItemIdBr, ItemTypeId,
    ctl::shared::EffectModes, err::BackrefRenderError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct StanceChangeCmd {
    type_id: Option<ItemTypeId>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
pub enum StanceChangeCmdCtxAny {
    Fit(StanceChangeCmdCtxFit),
    Item(StanceChangeCmdCtxItem),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(untagged))]
pub enum StanceChangeCmdCtxAnyBr {
    Fit(StanceChangeCmdCtxFitBr),
    Item(StanceChangeCmdCtxItemBr),
}

// Extra context commands - fit
pub struct StanceChangeCmdCtxFit {
    fit_id: FitId,
    core: StanceChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct StanceChangeCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: StanceChangeCmd,
}

// Extra context commands - item
pub struct StanceChangeCmdCtxItem {
    item_id: ItemId,
    core: StanceChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct StanceChangeCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: StanceChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StanceChangeCmd {
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
impl StanceChangeCmd {
    pub(in crate::ctl) fn into_ctx_via_fit(self, fit_id: FitId) -> StanceChangeCmdCtxAny {
        StanceChangeCmdCtxAny::Fit(StanceChangeCmdCtxFit { fit_id, core: self })
    }
    pub(in crate::ctl) fn into_ctx_via_item(self, item_id: ItemId) -> StanceChangeCmdCtxAny {
        StanceChangeCmdCtxAny::Item(StanceChangeCmdCtxItem { item_id, core: self })
    }
    pub(in crate::ctl) fn into_ctx_br_via_fit(self, fit_id: impl Into<FitIdBr>) -> StanceChangeCmdCtxAnyBr {
        StanceChangeCmdCtxAnyBr::Fit(StanceChangeCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        })
    }
    pub(in crate::ctl) fn into_ctx_br_via_item(self, item_id: impl Into<ItemIdBr>) -> StanceChangeCmdCtxAnyBr {
        StanceChangeCmdCtxAnyBr::Item(StanceChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StanceChangeCmdCtxAnyBr {
    pub(in crate::ctl) fn render(self, resps: &CmdResps) -> Result<StanceChangeCmdCtxAny, BackrefRenderError> {
        Ok(match self {
            Self::Fit(cmd) => StanceChangeCmdCtxAny::Fit(cmd.render(resps)?),
            Self::Item(cmd) => StanceChangeCmdCtxAny::Item(cmd.render(resps)?),
        })
    }
}

impl StanceChangeCmdCtxFitBr {
    fn render(self, resps: &CmdResps) -> Result<StanceChangeCmdCtxFit, BackrefRenderError> {
        Ok(StanceChangeCmdCtxFit {
            fit_id: resps.render_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

impl StanceChangeCmdCtxItemBr {
    fn render(self, resps: &CmdResps) -> Result<StanceChangeCmdCtxItem, BackrefRenderError> {
        Ok(StanceChangeCmdCtxItem {
            item_id: resps.render_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StanceChangeCmd {
    pub(in crate::ctl) fn execute_via_fit(
        self,
        core_fit: &mut rc::FitMut,
    ) -> Result<ChangedItemIdsResp, FitStanceChangeError> {
        let mut core_stance = match core_fit.get_stance_mut() {
            Some(core_stance) => core_stance,
            None => return Err(FitStanceChangeError::FitNoStance(core_fit.get_fit_id())),
        };
        Ok(self.execute(&mut core_stance))
    }
    pub(in crate::ctl) fn execute_via_item(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemStanceChangeError> {
        let core_stance = core_item.dc_stance()?;
        Ok(self.execute(core_stance))
    }
    fn execute(self, core_stance: &mut rc::StanceMut) -> ChangedItemIdsResp {
        if let Some(type_id) = self.type_id {
            core_stance.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_stance.set_state(state);
        }
        self.effect_modes.apply(core_stance);
        ChangedItemIdsResp::default()
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitStanceChangeError {
    #[error("fit {0} has no stance set")]
    FitNoStance(FitId),
}
#[derive(thiserror::Error, Debug)]
pub enum ItemStanceChangeError {
    #[error(transparent)]
    ItemIsNotStance(#[from] rc::err::ItemKindMatchError),
}

impl StanceChangeCmdCtxAny {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, StanceChangeError> {
        match self {
            Self::Fit(cmd) => Ok(cmd.execute(core_sol)?),
            Self::Item(cmd) => Ok(cmd.execute(core_sol)?),
        }
    }
}
#[derive(thiserror::Error, Debug)]
pub enum StanceChangeError {
    #[error(transparent)]
    ViaFit(#[from] FitGetStanceChangeError),
    #[error(transparent)]
    ViaItem(#[from] ItemGetStanceChangeError),
}

impl StanceChangeCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, FitGetStanceChangeError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute_via_fit(&mut core_fit)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetStanceChangeError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
    #[error("fit {0} has no stance set")]
    FitNoStance(FitId),
}
impl From<FitStanceChangeError> for FitGetStanceChangeError {
    fn from(err: FitStanceChangeError) -> Self {
        match err {
            FitStanceChangeError::FitNoStance(inner) => Self::FitNoStance(inner),
        }
    }
}

impl StanceChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetStanceChangeError> {
        let mut core_stance = core_sol.get_stance_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_stance))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetStanceChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetStanceError),
}
