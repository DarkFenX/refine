use crate::{
    AbilityId, AddedItemIdsResp, CmdResps, Coordinates, CountNz, EffectId, EffectMode, FitId, FitIdBr, ItemId,
    ItemIdBr, ItemTypeId, MinionState, Movement, RearmMinion,
    ctl::core::shared::{Abilities, EffectModes},
    err::BrResolveError,
    shared::CmdResidue,
};

// Core commands
pub type FighterAddCmd = FighterAddCmdGen<ItemId>;
pub type FighterAddCmdBr = FighterAddCmdGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FighterAddCmdGen<I> {
    type_id: ItemTypeId,
    state: MinionState,
    count: Option<CountNz> = None,
    rearm_minion: Option<RearmMinion> = None,
    coordinates: Option<Coordinates> = None,
    movement: Option<Movement> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    proj_item_ids: Vec<I> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    abilities: Abilities = Abilities::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
pub type FighterAddCmdCtxFit = FighterAddCmdCtxFitGen<FitId, ItemId>;
pub type FighterAddCmdCtxFitBr = FighterAddCmdCtxFitGen<FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FighterAddCmdCtxFitGen<F, I> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FighterAddCmdGen<I>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> FighterAddCmdGen<I> {
    pub fn new(type_id: ItemTypeId, state: MinionState) -> Self {
        Self { type_id, state, .. }
    }
    pub fn with_count(mut self, count: CountNz) -> Self {
        self.count = Some(count);
        self
    }
    pub fn with_abilities(mut self, abilities: impl IntoIterator<Item = (AbilityId, bool)>) -> Self {
        self.abilities.extend(abilities);
        self
    }
    pub fn with_rearm_minion(mut self, rearm_minion: RearmMinion) -> Self {
        self.rearm_minion = Some(rearm_minion);
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
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl IntoIterator<Item = I>) -> Self {
        self.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl IntoIterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
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
            type_id: self.type_id,
            state: self.state,
            count: self.count,
            abilities: self.abilities,
            rearm_minion: self.rearm_minion,
            coordinates: self.coordinates,
            movement: self.movement,
            effect_modes: self.effect_modes,
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
impl<I> FighterAddCmdGen<I> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        match self.proj_item_ids.is_empty() {
            true => CmdResidue::MutInfallible,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}
impl<F, I> FighterAddCmdCtxFitGen<F, I> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        match self.core.proj_item_ids.is_empty() {
            true => CmdResidue::MutFallibleClean,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}

impl FighterAddCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, FighterAddError> {
        let mut core_fighter = core_fit.add_fighter(self.type_id, self.state, self.coordinates, self.movement);
        if let Some(count) = self.count {
            core_fighter.set_count_override(Some(count));
        }
        self.abilities.apply(&mut core_fighter);
        if let Some(rearm_minion) = self.rearm_minion {
            core_fighter.set_rearm_minion(Some(rearm_minion));
        }
        self.effect_modes.apply(&mut core_fighter);
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
