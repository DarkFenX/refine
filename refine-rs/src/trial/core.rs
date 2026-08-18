use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr, ItemTypeId, err::BrResolveError, shared::val_options_br_resolve,
    val::ValOptions,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitTryItemsCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    val_options: ValOptions<ItemId>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitTryItemsCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitTryItemsCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    val_options: ValOptions<ItemIdBr>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitTryItemsCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
struct FitTryItemsCmdShared {
    type_ids: Vec<ItemTypeId>,
}

// Extra context commands
#[derive(Clone)]
pub struct FitTryItemsCmdCtxFit {
    fit_id: FitId,
    core: FitTryItemsCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FitTryItemsCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitTryItemsCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitTryItemsCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_val_options(mut self, val_options: ValOptions<ItemId>) -> Self {
        self.val_options = val_options;
        self
    }
    pub fn with_type_ids(mut self, type_ids: impl Iterator<Item = ItemTypeId>) -> Self {
        self.shared.type_ids.extend(type_ids);
        self
    }
}

impl FitTryItemsCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_val_options(mut self, val_options: ValOptions<ItemIdBr>) -> Self {
        self.val_options = val_options;
        self
    }
    pub fn with_type_ids(mut self, type_ids: impl Iterator<Item = ItemTypeId>) -> Self {
        self.shared.type_ids.extend(type_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitTryItemsCmdBr {
    pub(in crate::trial) fn into_ctx_item_br(self, fit_id: impl Into<FitIdBr>) -> FitTryItemsCmdCtxFitBr {
        FitTryItemsCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitTryItemsCmdBr {
    pub(in crate::trial) fn br_resolve(self, resps: &CmdResps) -> Result<FitTryItemsCmd, BrResolveError> {
        Ok(FitTryItemsCmd {
            val_options: val_options_br_resolve(self.val_options, resps)?,
            shared: self.shared,
        })
    }
}

impl FitTryItemsCmdCtxFitBr {
    pub(in crate::trial) fn br_resolve(self, resps: &CmdResps) -> Result<FitTryItemsCmdCtxFit, BrResolveError> {
        Ok(FitTryItemsCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitTryItemsCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> Vec<ItemTypeId> {
        core_fit.try_fit_items(&self.shared.type_ids, &self.val_options)
    }
}

impl FitTryItemsCmdCtxFit {
    pub(in crate::trial) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<Vec<ItemTypeId>, FitGetFitValError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetFitValError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
}
