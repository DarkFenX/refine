use crate::{CmdResps, FitId, FitIdBr, err::BrResolveError, shared::CmdResidue};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ShipUnsetCmd;

// Extra context commands
pub type ShipUnsetCmdCtxFit = ShipUnsetCmdCtxFitGen<FitId>;
pub type ShipUnsetCmdCtxFitBr = ShipUnsetCmdCtxFitGen<FitIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ShipUnsetCmdCtxFitGen<F> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ShipUnsetCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipUnsetCmd {
    pub fn new() -> Self {
        Self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipUnsetCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> ShipUnsetCmdCtxFit {
        ShipUnsetCmdCtxFit { fit_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> ShipUnsetCmdCtxFitBr {
        ShipUnsetCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipUnsetCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ShipUnsetCmdCtxFit, BrResolveError> {
        Ok(ShipUnsetCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipUnsetCmd {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutInfallible
    }
}
impl<F> ShipUnsetCmdCtxFitGen<F> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}

impl ShipUnsetCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) {
        if let Some(core_ship) = core_fit.get_ship_mut() {
            core_ship.remove();
        }
    }
}

impl ShipUnsetCmdCtxFit {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), FitGetShipUnsetError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        self.core.execute(&mut core_fit);
        Ok(())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetShipUnsetError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
}
