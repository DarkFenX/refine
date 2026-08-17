use crate::{
    AbilityId, ChangedItemIdsResp, CmdResps, Coordinates, CountNz, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId,
    MinionState, Movement, RearmMinion, TriStateField,
    ctl::core::shared::{Abilities, EffectModes},
    err::BrResolveError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FighterChangeCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    add_proj_item_ids: Vec<ItemId>,
    #[cfg_attr(feature = "serde", serde(default))]
    rm_proj_item_ids: Vec<ItemId>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FighterChangeCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FighterChangeCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    add_proj_item_ids: Vec<ItemIdBr>,
    #[cfg_attr(feature = "serde", serde(default))]
    rm_proj_item_ids: Vec<ItemIdBr>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FighterChangeCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
struct FighterChangeCmdShared {
    type_id: Option<ItemTypeId>,
    state: Option<MinionState>,
    #[cfg_attr(feature = "serde", serde(default))]
    count: TriStateField<CountNz>,
    #[cfg_attr(feature = "serde", serde(default))]
    abilities: Abilities,
    #[cfg_attr(feature = "serde", serde(default))]
    rearm_minion: TriStateField<RearmMinion>,
    coordinates: Option<Coordinates>,
    movement: Option<Movement>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FighterChangeCmdCtxItem {
    item_id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FighterChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FighterChangeCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FighterChangeCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FighterChangeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.shared.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: MinionState) -> Self {
        self.shared.state = Some(state);
        self
    }
    pub fn with_count(mut self, count: Option<CountNz>) -> Self {
        self.shared.count = count.into();
        self
    }
    pub fn with_abilities(mut self, abilities: impl Iterator<Item = (AbilityId, bool)>) -> Self {
        self.shared.abilities.extend(abilities);
        self
    }
    pub fn with_rearm_minion(mut self, rearm_minion: Option<RearmMinion>) -> Self {
        self.shared.rearm_minion = rearm_minion.into();
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
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.rm_proj_item_ids.extend(rm_proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.shared.effect_modes.extend(effect_modes);
        self
    }
}

impl FighterChangeCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.shared.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: MinionState) -> Self {
        self.shared.state = Some(state);
        self
    }
    pub fn with_count(mut self, count: Option<CountNz>) -> Self {
        self.shared.count = count.into();
        self
    }
    pub fn with_abilities(mut self, abilities: impl Iterator<Item = (AbilityId, bool)>) -> Self {
        self.shared.abilities.extend(abilities);
        self
    }
    pub fn with_rearm_minion(mut self, rearm_minion: Option<RearmMinion>) -> Self {
        self.shared.rearm_minion = rearm_minion.into();
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
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.rm_proj_item_ids.extend(rm_proj_item_ids);
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
impl FighterChangeCmd {
    pub(in crate::ctl) fn into_ctx_item(self, item_id: ItemId) -> FighterChangeCmdCtxItem {
        FighterChangeCmdCtxItem { item_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> FighterChangeCmdCtxItemBr {
        FighterChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self.into_br(),
        }
    }
    fn into_br(self) -> FighterChangeCmdBr {
        FighterChangeCmdBr {
            add_proj_item_ids: self.add_proj_item_ids.into_iter().map(ItemIdBr::Id).collect(),
            rm_proj_item_ids: self.rm_proj_item_ids.into_iter().map(ItemIdBr::Id).collect(),
            shared: self.shared,
        }
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
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FighterChangeCmdCtxItemBr {
    pub(in crate::ctl) fn render(self, resps: &CmdResps) -> Result<FighterChangeCmdCtxItem, BrResolveError> {
        Ok(FighterChangeCmdCtxItem {
            item_id: resps.render_item_id(self.item_id)?,
            core: self.core.render(resps)?,
        })
    }
}

impl FighterChangeCmdBr {
    fn render(self, resps: &CmdResps) -> Result<FighterChangeCmd, BrResolveError> {
        Ok(FighterChangeCmd {
            add_proj_item_ids: resps.render_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.render_item_ids(self.rm_proj_item_ids)?,
            shared: self.shared,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FighterChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, FighterChangeError> {
        let core_fighter = core_item.dc_fighter()?;
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_fighter.get_proj_mut(projectee_item_id)?.remove();
        }
        if let Some(type_id) = self.shared.type_id {
            core_fighter.set_type_id(type_id);
        }
        if let Some(state) = self.shared.state {
            core_fighter.set_state(state);
        }
        match self.shared.count {
            TriStateField::Value(count) => core_fighter.set_count_override(Some(count)),
            TriStateField::None => core_fighter.set_count_override(None),
            TriStateField::Absent => (),
        }
        self.shared.abilities.apply(core_fighter);
        match self.shared.rearm_minion {
            TriStateField::Value(rearm_minion) => core_fighter.set_rearm_minion(Some(rearm_minion)),
            TriStateField::None => core_fighter.set_rearm_minion(None),
            TriStateField::Absent => (),
        }
        if let Some(coordinates) = self.shared.coordinates {
            core_fighter.set_coordinates(coordinates);
        }
        if let Some(movement) = self.shared.movement {
            core_fighter.set_movement(movement);
        }
        self.shared.effect_modes.apply(core_fighter);
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
    ProjAdd(#[from] rc::err::AddProjError),
    #[error("unable to remove projection")]
    ProjRemove(#[from] rc::err::GetProjError),
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
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotFighter(rc::err::ItemKindMatchError),
    #[error("unable to add projection")]
    ProjAdd(#[source] rc::err::AddProjError),
    #[error("unable to remove projection")]
    ProjRemove(#[source] rc::err::GetProjError),
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
