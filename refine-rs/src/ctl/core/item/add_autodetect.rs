use crate::{AddedItemIdsResp, CmdResps, FitId, FitIdBr, ItemTypeId, err::BrResolveError, shared::CmdResidue};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ItemAutodetectAddCmd {
    type_id: ItemTypeId,
}

// Extra context commands
pub type ItemAutodetectAddCmdCtxFit = ItemAutodetectAddCmdCtxFitGen<FitId>;
pub type ItemAutodetectAddCmdCtxFitBr = ItemAutodetectAddCmdCtxFitGen<FitIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ItemAutodetectAddCmdCtxFitGen<F> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ItemAutodetectAddCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAutodetectAddCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self { type_id }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAutodetectAddCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> ItemAutodetectAddCmdCtxFit {
        ItemAutodetectAddCmdCtxFit { fit_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> ItemAutodetectAddCmdCtxFitBr {
        ItemAutodetectAddCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAutodetectAddCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ItemAutodetectAddCmdCtxFit, BrResolveError> {
        Ok(ItemAutodetectAddCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAutodetectAddCmd {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}
impl<F> ItemAutodetectAddCmdCtxFitGen<F> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}

impl ItemAutodetectAddCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, ItemAutodetectAddError> {
        let core_item = core_fit.autodetect_add_item(self.type_id)?;
        Ok(AddedItemIdsResp::from_core_item(core_item))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemAutodetectAddError {
    #[error(transparent)]
    ItemAutodetectAdd(#[from] rc::err::FitItemAutodetectAddError),
}

impl ItemAutodetectAddCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, FitGetItemAutodetectAddError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetItemAutodetectAddError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
    #[error(transparent)]
    ItemAutodetectAdd(rc::err::FitItemAutodetectAddError),
}
impl From<ItemAutodetectAddError> for FitGetItemAutodetectAddError {
    fn from(err: ItemAutodetectAddError) -> Self {
        match err {
            ItemAutodetectAddError::ItemAutodetectAdd(inner) => Self::ItemAutodetectAdd(inner),
        }
    }
}
