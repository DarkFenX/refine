use crate::{
    AddItemEnumCmd,
    cmd::inner::{ICmdFighterAddFCtxRIds, ICmdFighterAddICtxRIds, ICmdFighterAddShared},
};

pub struct ItemAddFighterCmd {
    pub(super) inner: ICmdFighterAddFCtxRIds,
}
impl ItemAddFighterCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId, state: rc::MinionState) -> Self {
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
    pub fn with_count(mut self, count: rc::CountNz) -> Self {
        self.inner.ictx_cmd.shared.count = Some(count);
        self
    }
    pub fn with_abilities(mut self, abilities: impl Iterator<Item = (rc::AbilityId, bool)>) -> Self {
        self.inner.ictx_cmd.shared.abilities.clear();
        self.inner.ictx_cmd.shared.abilities.extend(abilities);
        self
    }
    pub fn with_rearm_minion(mut self, rearm_minion: rc::RearmMinion) -> Self {
        self.inner.ictx_cmd.shared.rearm_minion = Some(rearm_minion);
        self
    }
    pub fn with_coordinates(mut self, coordinates: rc::Coordinates) -> Self {
        self.inner.ictx_cmd.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: rc::Movement) -> Self {
        self.inner.ictx_cmd.shared.movement = Some(movement);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = rc::ItemId>) -> Self {
        self.inner.ictx_cmd.proj_item_ids.clear();
        self.inner.ictx_cmd.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
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
