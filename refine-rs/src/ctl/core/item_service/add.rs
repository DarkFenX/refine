use crate::{
    AddedItemIdsResp, CmdResps, EffectId, EffectMode, FitId, FitIdBr, ItemTypeId, ServiceState,
    ctl::core::shared::EffectModes, err::BrResolveError, shared::CmdResidue,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ServiceAddCmd {
    type_id: ItemTypeId,
    state: ServiceState,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
pub type ServiceAddCmdCtxFit = ServiceAddCmdCtxFitGen<FitId>;
pub type ServiceAddCmdCtxFitBr = ServiceAddCmdCtxFitGen<FitIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ServiceAddCmdCtxFitGen<F> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ServiceAddCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ServiceAddCmd {
    pub fn new(type_id: ItemTypeId, state: ServiceState) -> Self {
        Self { type_id, state, .. }
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ServiceAddCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> ServiceAddCmdCtxFit {
        ServiceAddCmdCtxFit { fit_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> ServiceAddCmdCtxFitBr {
        ServiceAddCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ServiceAddCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ServiceAddCmdCtxFit, BrResolveError> {
        Ok(ServiceAddCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ServiceAddCmd {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutInfallible
    }
}
impl<F> ServiceAddCmdCtxFitGen<F> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}

impl ServiceAddCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_service = core_fit.add_service(self.type_id, self.state);
        self.effect_modes.apply(&mut core_service);
        AddedItemIdsResp::from_core_service(core_service)
    }
}

impl ServiceAddCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, FitGetServiceAddError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetServiceAddError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
}
