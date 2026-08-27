use crate::{
    AbilityId, AddedItemIdsResp, CmdResps, Coordinates, CountNz, EffectId, EffectMode, FitId, FitIdBr, ItemId,
    ItemIdBr, ItemTypeId, MinionState, Movement, RearmMinion,
    ctl::core::shared::{Abilities, EffectModes},
    err::BrResolveError,
    shared::CmdResidue,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FighterAddCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    proj_item_ids: Vec<ItemId> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FighterAddCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FighterAddCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    proj_item_ids: Vec<ItemIdBr> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FighterAddCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
struct FighterAddCmdShared {
    type_id: ItemTypeId,
    state: MinionState,
    count: Option<CountNz> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    abilities: Abilities = Abilities::new(),
    rearm_minion: Option<RearmMinion> = None,
    coordinates: Option<Coordinates> = None,
    movement: Option<Movement> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FighterAddCmdCtxFit {
    fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FighterAddCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FighterAddCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FighterAddCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FighterAddCmd {
    pub fn new(type_id: ItemTypeId, state: MinionState) -> Self {
        Self {
            shared: FighterAddCmdShared { type_id, state, .. },
            ..
        }
    }
    pub fn with_count(mut self, count: CountNz) -> Self {
        self.shared.count = Some(count);
        self
    }
    pub fn with_abilities(mut self, abilities: impl Iterator<Item = (AbilityId, bool)>) -> Self {
        self.shared.abilities.extend(abilities);
        self
    }
    pub fn with_rearm_minion(mut self, rearm_minion: RearmMinion) -> Self {
        self.shared.rearm_minion = Some(rearm_minion);
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.shared.movement = Some(movement);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.shared.effect_modes.extend(effect_modes);
        self
    }
}

impl FighterAddCmdBr {
    pub fn new(type_id: ItemTypeId, state: MinionState) -> Self {
        Self {
            shared: FighterAddCmdShared { type_id, state, .. },
            ..
        }
    }
    pub fn with_count(mut self, count: CountNz) -> Self {
        self.shared.count = Some(count);
        self
    }
    pub fn with_abilities(mut self, abilities: impl Iterator<Item = (AbilityId, bool)>) -> Self {
        self.shared.abilities.extend(abilities);
        self
    }
    pub fn with_rearm_minion(mut self, rearm_minion: RearmMinion) -> Self {
        self.shared.rearm_minion = Some(rearm_minion);
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.shared.movement = Some(movement);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.shared.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FighterAddCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> FighterAddCmdCtxFit {
        FighterAddCmdCtxFit { fit_id, core: self }
    }
}

impl FighterAddCmdBr {
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> FighterAddCmdCtxFitBr {
        FighterAddCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FighterAddCmdBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<FighterAddCmd, BrResolveError> {
        Ok(FighterAddCmd {
            proj_item_ids: resps.resolve_item_ids(self.proj_item_ids)?,
            shared: self.shared,
        })
    }
}

impl FighterAddCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<FighterAddCmdCtxFit, BrResolveError> {
        Ok(FighterAddCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FighterAddCmd {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self.proj_item_ids.is_empty() {
            true => CmdResidue::MutInfallible,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}
impl FighterAddCmdBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self.proj_item_ids.is_empty() {
            true => CmdResidue::MutInfallible,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}
impl FighterAddCmdCtxFit {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self.core.proj_item_ids.is_empty() {
            true => CmdResidue::MutFallibleClean,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}
impl FighterAddCmdCtxFitBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self.core.proj_item_ids.is_empty() {
            true => CmdResidue::MutFallibleClean,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}

impl FighterAddCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, FighterAddError> {
        let mut core_fighter = core_fit.add_fighter(
            self.shared.type_id,
            self.shared.state,
            self.shared.coordinates,
            self.shared.movement,
        );
        if let Some(count) = self.shared.count {
            core_fighter.set_count_override(Some(count));
        }
        self.shared.abilities.apply(&mut core_fighter);
        if let Some(rearm_minion) = self.shared.rearm_minion {
            core_fighter.set_rearm_minion(Some(rearm_minion));
        }
        self.shared.effect_modes.apply(&mut core_fighter);
        for projectee_item_id in self.proj_item_ids.iter() {
            core_fighter.add_proj(projectee_item_id)?;
        }
        Ok(AddedItemIdsResp::from_core_fighter(core_fighter))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FighterAddError {
    #[error("failed to add projection")]
    ProjAdd(#[from] rc::err::ProjAddError),
}

impl FighterAddCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, FitGetFighterAddError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetFighterAddError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
    #[error("failed to add projection")]
    ProjAdd(#[source] rc::err::ProjAddError),
}
impl From<FighterAddError> for FitGetFighterAddError {
    fn from(err: FighterAddError) -> Self {
        match err {
            FighterAddError::ProjAdd(inner) => Self::ProjAdd(inner),
        }
    }
}
