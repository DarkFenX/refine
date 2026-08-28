use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr, ItemTypeId, err::BrResolveError, shared::CmdResidue, val::ValOptions,
};

// Core commands
pub type FitTryItemsCmd = FitTryItemsCmdGen<ItemId>;
pub type FitTryItemsCmdBr = FitTryItemsCmdGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FitTryItemsCmdGen<I> {
    #[cfg_attr(feature = "serde", serde(default))]
    val_options: ValOptions<I>,
    type_ids: Vec<ItemTypeId>,
}
impl<I> Default for FitTryItemsCmdGen<I> {
    fn default() -> Self {
        Self {
            val_options: Default::default(),
            type_ids: Default::default(),
        }
    }
}

// Extra context commands
pub type FitTryItemsCmdCtxFit = FitTryItemsCmdCtxFitGen<FitId, ItemId>;
pub type FitTryItemsCmdCtxFitBr = FitTryItemsCmdCtxFitGen<FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FitTryItemsCmdCtxFitGen<F, I> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitTryItemsCmdGen<I>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> FitTryItemsCmdGen<I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_val_options(mut self, val_options: ValOptions<I>) -> Self {
        self.val_options = val_options;
        self
    }
    pub fn with_type_ids(mut self, type_ids: impl IntoIterator<Item = ItemTypeId>) -> Self {
        self.type_ids.extend(type_ids);
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
    pub(in crate::trial) fn br_resolve(self, resps: &CmdResps) -> FitTryItemsCmd {
        FitTryItemsCmd {
            val_options: self
                .val_options
                .filter_map_item_ids(|item_id_br| resps.resolve_item_id(item_id_br).ok()),
            type_ids: self.type_ids,
        }
    }
}

impl FitTryItemsCmdCtxFitBr {
    pub(in crate::trial) fn br_resolve(self, resps: &CmdResps) -> Result<FitTryItemsCmdCtxFit, BrResolveError> {
        Ok(FitTryItemsCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core.br_resolve(resps),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> FitTryItemsCmdGen<I> {
    pub(in crate::trial) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutInfallible
    }
}
impl<F, I> FitTryItemsCmdCtxFitGen<F, I> {
    pub(in crate::trial) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutFallible
    }
}

impl FitTryItemsCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> Vec<ItemTypeId> {
        // Execute in cloned sol, since at the present time try_fit_items() changes its state
        let fit_id = core_fit.get_fit_id();
        let mut cloned_sol = core_fit.get_sol().clone();
        let mut cloned_fit = cloned_sol.get_fit_mut(&fit_id).unwrap();
        cloned_fit.try_fit_items(&self.type_ids, &self.val_options)
    }
}

impl FitTryItemsCmdCtxFit {
    pub(in crate::trial) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<Vec<ItemTypeId>, FitGetFitTryItemsError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetFitTryItemsError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
}
