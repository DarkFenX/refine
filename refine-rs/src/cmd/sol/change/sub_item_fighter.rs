use crate::{
    AbilityId, ChangeSolEnumCmd, Coordinates, CountNz, EffectId, EffectMode, FitIdBackref, ItemIdBackref, ItemTypeId,
    MinionState, Movement, RearmMinion,
    cmd::inner::{ICmdFighterAddFCtxBIds, ICmdFighterAddICtxBIds, ICmdFighterAddShared, ICmdFighterChangeFCtxBIds},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Add
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolAddFighterCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdFighterAddFCtxBIds,
}
impl SolAddFighterCmd {
    pub fn new(fit_id: FitIdBackref, type_id: ItemTypeId, state: MinionState) -> Self {
        Self {
            inner: ICmdFighterAddFCtxBIds {
                fit_id,
                ictx_cmd: ICmdFighterAddICtxBIds {
                    shared: ICmdFighterAddShared { type_id, state, .. },
                    ..
                },
            },
        }
    }
    pub fn with_count(mut self, count: CountNz) -> Self {
        self.inner.ictx_cmd.shared.count = Some(count);
        self
    }
    pub fn with_abilities(mut self, abilities: impl Iterator<Item = (AbilityId, bool)>) -> Self {
        self.inner.ictx_cmd.shared.abilities.clear();
        self.inner.ictx_cmd.shared.abilities.extend(abilities);
        self
    }
    pub fn with_rearm_minion(mut self, rearm_minion: RearmMinion) -> Self {
        self.inner.ictx_cmd.shared.rearm_minion = Some(rearm_minion);
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.inner.ictx_cmd.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.inner.ictx_cmd.shared.movement = Some(movement);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemIdBackref>) -> Self {
        self.inner.ictx_cmd.proj_item_ids.clear();
        self.inner.ictx_cmd.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.shared.effect_modes.clear();
        self.inner.ictx_cmd.shared.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolAddFighterCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddFighterCmd) -> Self {
        Self::AddFighter(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeFighterCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdFighterChangeFCtxBIds,
}
impl SolChangeFighterCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdFighterChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.ictx_cmd.shared.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: MinionState) -> Self {
        self.inner.ictx_cmd.shared.state = Some(state);
        self
    }
    pub fn with_count(mut self, count: Option<CountNz>) -> Self {
        self.inner.ictx_cmd.shared.count = count.into();
        self
    }
    pub fn with_abilities(mut self, abilities: impl Iterator<Item = (AbilityId, bool)>) -> Self {
        self.inner.ictx_cmd.shared.abilities.clear();
        self.inner.ictx_cmd.shared.abilities.extend(abilities);
        self
    }
    pub fn with_rearm_minion(mut self, rearm_minion: Option<RearmMinion>) -> Self {
        self.inner.ictx_cmd.shared.rearm_minion = rearm_minion.into();
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.inner.ictx_cmd.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.inner.ictx_cmd.shared.movement = Some(movement);
        self
    }
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl Iterator<Item = ItemIdBackref>) -> Self {
        self.inner.ictx_cmd.add_proj_item_ids.clear();
        self.inner.ictx_cmd.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl Iterator<Item = ItemIdBackref>) -> Self {
        self.inner.ictx_cmd.rm_proj_item_ids.clear();
        self.inner.ictx_cmd.rm_proj_item_ids.extend(rm_proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.shared.effect_modes.clear();
        self.inner.ictx_cmd.shared.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolChangeFighterCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeFighterCmd) -> Self {
        Self::ChangeFighter(sub_cmd)
    }
}
