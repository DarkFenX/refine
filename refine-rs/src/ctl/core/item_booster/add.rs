use crate::{
    AddedItemIdsResp, CmdResps, EffectId, EffectMode, FitId, FitIdBr, ItemTypeId,
    ctl::core::shared::{EffectModes, SideEffects},
    err::BrResolveError,
    shared::CmdResidue,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct BoosterAddCmd {
    type_id: ItemTypeId,
    state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    side_effects: SideEffects = SideEffects::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
pub type BoosterAddCmdCtxFit = BoosterAddCmdCtxFitGen<FitId>;
pub type BoosterAddCmdCtxFitBr = BoosterAddCmdCtxFitGen<FitIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct BoosterAddCmdCtxFitGen<F> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: BoosterAddCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BoosterAddCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self { type_id, .. }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.state = Some(state);
        self
    }
    pub fn with_side_effects(mut self, side_effects: impl Iterator<Item = (EffectId, bool)>) -> Self {
        self.side_effects.extend(side_effects);
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
impl BoosterAddCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> BoosterAddCmdCtxFit {
        BoosterAddCmdCtxFit { fit_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> BoosterAddCmdCtxFitBr {
        BoosterAddCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BoosterAddCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<BoosterAddCmdCtxFit, BrResolveError> {
        Ok(BoosterAddCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BoosterAddCmd {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutInfallible
    }
}

impl<F> BoosterAddCmdCtxFitGen<F> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}

impl BoosterAddCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_booster = core_fit.add_booster(self.type_id);
        if let Some(state) = self.state {
            core_booster.set_state(state);
        }
        self.side_effects.apply(&mut core_booster);
        self.effect_modes.apply(&mut core_booster);
        AddedItemIdsResp::from_core_booster(core_booster)
    }
}

impl BoosterAddCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, FitGetBoosterAddError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetBoosterAddError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
}
