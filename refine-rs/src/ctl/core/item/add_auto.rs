use crate::{AddedItemIdsResp, CmdResps, FitId, FitIdBr, ItemTypeId, err::BrResolveError, shared::CmdResidue};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ItemAddAutoCmd {
    type_id: ItemTypeId,
}

// Extra context commands
pub type ItemAddAutoCmdCtxFit = ItemAddAutoCmdCtxFitGen<FitId>;
pub type ItemAddAutoCmdCtxFitBr = ItemAddAutoCmdCtxFitGen<FitIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ItemAddAutoCmdCtxFitGen<F> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ItemAddAutoCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAddAutoCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self { type_id }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAddAutoCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> ItemAddAutoCmdCtxFit {
        ItemAddAutoCmdCtxFit { fit_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> ItemAddAutoCmdCtxFitBr {
        ItemAddAutoCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAddAutoCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ItemAddAutoCmdCtxFit, BrResolveError> {
        Ok(ItemAddAutoCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAddAutoCmd {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}
impl<F> ItemAddAutoCmdCtxFitGen<F> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}

impl ItemAddAutoCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, ItemAddAutoError> {
        let core_item = core_fit.add_item_auto(self.type_id)?;
        Ok(AddedItemIdsResp::from_core_item(core_item))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemAddAutoError {
    #[error(transparent)]
    ItemAddAuto(#[from] rc::err::FitAddItemAutoError),
}

impl ItemAddAutoCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, FitGetItemAddAutoError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetItemAddAutoError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
    #[error(transparent)]
    ItemAddAuto(rc::err::FitAddItemAutoError),
}
impl From<ItemAddAutoError> for FitGetItemAddAutoError {
    fn from(err: ItemAddAutoError) -> Self {
        match err {
            ItemAddAutoError::ItemAddAuto(inner) => Self::ItemAddAuto(inner),
        }
    }
}
