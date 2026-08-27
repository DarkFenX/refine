use crate::{
    AddedItemIdsResp, CmdResps, Coordinates, EffectId, EffectMode, FitId, FitIdBr, ItemTypeId, Movement,
    ctl::core::shared::EffectModes, err::BrResolveError, shared::CmdResidue,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ShipSetCmd {
    type_id: ItemTypeId,
    state: Option<bool> = None,
    coordinates: Option<Coordinates> = None,
    movement: Option<Movement> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ShipSetCmdCtxFit {
    fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ShipSetCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ShipSetCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ShipSetCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipSetCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self { type_id, .. }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.state = Some(state);
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.movement = Some(movement);
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
impl ShipSetCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> ShipSetCmdCtxFit {
        ShipSetCmdCtxFit { fit_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> ShipSetCmdCtxFitBr {
        ShipSetCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipSetCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ShipSetCmdCtxFit, BrResolveError> {
        Ok(ShipSetCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipSetCmd {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutInfallible
    }
}
impl ShipSetCmdCtxFit {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}
impl ShipSetCmdCtxFitBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}

impl ShipSetCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_ship = core_fit.set_ship(self.type_id, self.coordinates, self.movement);
        if let Some(state) = self.state {
            core_ship.set_state(state);
        }
        self.effect_modes.apply(&mut core_ship);
        AddedItemIdsResp::from_core_ship(core_ship)
    }
}

impl ShipSetCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, FitGetShipSetError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetShipSetError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
}
