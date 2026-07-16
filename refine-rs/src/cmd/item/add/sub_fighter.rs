use crate::{
    AbilityId, AddItemEnumCmd, Coordinates, CountNz, EffectId, EffectMode, FitId, ItemId, ItemTypeId, MinionState,
    Movement, RearmMinion,
    cmd::inner::{ICmdFighterAddFCtxRIds, ICmdFighterAddICtxRIds, ICmdFighterAddShared},
};

pub struct ItemAddFighterCmd {
    pub(super) inner: ICmdFighterAddFCtxRIds,
}
impl ItemAddFighterCmd {
    pub fn new(fit_id: FitId, type_id: ItemTypeId, state: MinionState) -> Self {
        Self {
            inner: ICmdFighterAddFCtxRIds {
                fit_id,
                ictx_cmd: ICmdFighterAddICtxRIds {
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
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
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
impl From<ItemAddFighterCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddFighterCmd) -> Self {
        Self::Fighter(sub_cmd)
    }
}
