use crate::cmd::{ChangeItemEnumCmd, inner::ICmdFighterChangeICtxRIds};

#[derive(Default)]
pub struct ItemChangeFighterCmd {
    pub(super) inner: ICmdFighterChangeICtxRIds = ICmdFighterChangeICtxRIds { .. },
}
impl ItemChangeFighterCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.shared.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: rc::MinionState) -> Self {
        self.inner.shared.state = Some(state);
        self
    }
    pub fn with_count(mut self, count: Option<rc::CountNz>) -> Self {
        self.inner.shared.count = count.into();
        self
    }
    pub fn with_abilities(mut self, abilities: impl Iterator<Item = (rc::AbilityId, bool)>) -> Self {
        self.inner.shared.abilities.clear();
        self.inner.shared.abilities.extend(abilities);
        self
    }
    pub fn with_rearm_minion(mut self, rearm_minion: Option<rc::RearmMinion>) -> Self {
        self.inner.shared.rearm_minion = rearm_minion.into();
        self
    }
    pub fn with_coordinates(mut self, coordinates: rc::Coordinates) -> Self {
        self.inner.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: rc::Movement) -> Self {
        self.inner.shared.movement = Some(movement);
        self
    }
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl Iterator<Item = rc::ItemId>) -> Self {
        self.inner.add_proj_item_ids.clear();
        self.inner.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl Iterator<Item = rc::ItemId>) -> Self {
        self.inner.rm_proj_item_ids.clear();
        self.inner.rm_proj_item_ids.extend(rm_proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.shared.effect_modes.clear();
        self.inner.shared.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemChangeFighterCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeFighterCmd) -> Self {
        Self::Fighter(sub_cmd)
    }
}
