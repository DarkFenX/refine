use crate::{
    AbilityId, ChangedItemIdsResp, CmdResps, Coordinates, CountNz, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId,
    MinionState, Movement, RearmMinion, TriStateField,
    ctl::core::shared::{Abilities, EffectModes},
    err::BrResolveError,
    shared::CmdResidue,
};

// Core commands
pub type FighterChangeCmd = FighterChangeCmdGen<ItemId>;
pub type FighterChangeCmdBr = FighterChangeCmdGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FighterChangeCmdGen<I> {
    type_id: Option<ItemTypeId>,
    state: Option<MinionState>,
    #[cfg_attr(feature = "serde", serde(default))]
    count_override: TriStateField<CountNz>,
    #[cfg_attr(feature = "serde", serde(default))]
    rearm_minion: TriStateField<RearmMinion>,
    coordinates: Option<Coordinates>,
    movement: Option<Movement>,
    #[cfg_attr(feature = "serde", serde(default))]
    add_proj_item_ids: Vec<I>,
    #[cfg_attr(feature = "serde", serde(default))]
    rm_proj_item_ids: Vec<I>,
    #[cfg_attr(feature = "serde", serde(default))]
    abilities: Abilities,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}
impl<I> Default for FighterChangeCmdGen<I> {
    fn default() -> Self {
        Self {
            type_id: Default::default(),
            state: Default::default(),
            count_override: Default::default(),
            rearm_minion: Default::default(),
            coordinates: Default::default(),
            movement: Default::default(),
            add_proj_item_ids: Default::default(),
            rm_proj_item_ids: Default::default(),
            abilities: Default::default(),
            effect_modes: Default::default(),
        }
    }
}

// Extra context commands
pub type FighterChangeCmdCtxItem = FighterChangeCmdCtxItemGen<ItemId>;
pub type FighterChangeCmdCtxItemBr = FighterChangeCmdCtxItemGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FighterChangeCmdCtxItemGen<I> {
    item_id: I,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FighterChangeCmdGen<I>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> FighterChangeCmdGen<I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: MinionState) -> Self {
        self.state = Some(state);
        self
    }
    pub fn with_count_override(mut self, count_override: Option<CountNz>) -> Self {
        self.count_override = count_override.into();
        self
    }
    pub fn with_abilities(mut self, abilities: impl IntoIterator<Item = (AbilityId, bool)>) -> Self {
        self.abilities.extend(abilities);
        self
    }
    pub fn with_rearm_minion(mut self, rearm_minion: Option<RearmMinion>) -> Self {
        self.rearm_minion = rearm_minion.into();
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
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl IntoIterator<Item = I>) -> Self {
        self.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl IntoIterator<Item = I>) -> Self {
        self.rm_proj_item_ids.extend(rm_proj_item_ids);
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
impl FighterChangeCmd {
    pub(in crate::ctl) fn into_ctx_item(self, item_id: ItemId) -> FighterChangeCmdCtxItem {
        FighterChangeCmdCtxItem { item_id, core: self }
    }
}
impl FighterChangeCmdBr {
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> FighterChangeCmdCtxItemBr {
        FighterChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FighterChangeCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<FighterChangeCmd, BrResolveError> {
        Ok(FighterChangeCmd {
            add_proj_item_ids: resps.resolve_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.resolve_item_ids(self.rm_proj_item_ids)?,
            type_id: self.type_id,
            state: self.state,
            count_override: self.count_override,
            abilities: self.abilities,
            rearm_minion: self.rearm_minion,
            coordinates: self.coordinates,
            movement: self.movement,
            effect_modes: self.effect_modes,
        })
    }
}

impl FighterChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<FighterChangeCmdCtxItem, BrResolveError> {
        Ok(FighterChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> FighterChangeCmdGen<I> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        // Assume the command always mutates (even if it does not with none of fields set)
        if !self.rm_proj_item_ids.is_empty() || !self.add_proj_item_ids.is_empty() {
            return CmdResidue::MutFallibleDirty;
        }
        CmdResidue::MutFallibleClean
    }
}
impl<I> FighterChangeCmdCtxItemGen<I> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        self.core.exec_residue()
    }
}

impl FighterChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, FighterChangeError> {
        let core_fighter = core_item.dc_fighter()?;
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_fighter.get_proj_mut(projectee_item_id)?.remove();
        }
        if let Some(type_id) = self.type_id {
            core_fighter.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_fighter.set_state(state);
        }
        match self.count_override {
            TriStateField::Value(count_override) => core_fighter.set_count_override(Some(count_override)),
            TriStateField::None => core_fighter.set_count_override(None),
            TriStateField::Absent => (),
        }
        self.abilities.apply(core_fighter);
        match self.rearm_minion {
            TriStateField::Value(rearm_minion) => core_fighter.set_rearm_minion(Some(rearm_minion)),
            TriStateField::None => core_fighter.set_rearm_minion(None),
            TriStateField::Absent => (),
        }
        if let Some(coordinates) = self.coordinates {
            core_fighter.set_coordinates(coordinates);
        }
        if let Some(movement) = self.movement {
            core_fighter.set_movement(movement);
        }
        self.effect_modes.apply(core_fighter);
        for projectee_item_id in self.add_proj_item_ids.iter() {
            core_fighter.add_proj(projectee_item_id)?;
        }
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FighterChangeError {
    #[error(transparent)]
    ItemIsNotFighter(#[from] rc::err::ItemKindMatchError),
    #[error("unable to add projection")]
    ProjAdd(#[from] rc::err::ProjAddError),
    #[error("unable to remove projection")]
    ProjRemove(#[from] rc::err::ProjGetError),
}

impl FighterChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetFighterChangeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetFighterChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::ItemGetError),
    #[error(transparent)]
    ItemIsNotFighter(rc::err::ItemKindMatchError),
    #[error("unable to add projection")]
    ProjAdd(#[source] rc::err::ProjAddError),
    #[error("unable to remove projection")]
    ProjRemove(#[source] rc::err::ProjGetError),
}
impl From<FighterChangeError> for ItemGetFighterChangeError {
    fn from(err: FighterChangeError) -> Self {
        match err {
            FighterChangeError::ItemIsNotFighter(inner) => Self::ItemIsNotFighter(inner),
            FighterChangeError::ProjAdd(inner) => Self::ProjAdd(inner),
            FighterChangeError::ProjRemove(inner) => Self::ProjRemove(inner),
        }
    }
}
