use crate::{
    ChangeItemEnumCmd, Coordinates, EffectId, EffectMode, ItemTypeId, Movement, cmd::inner::ICmdShipChangeICtx,
};

#[derive(Default)]
pub struct ItemChangeShipCmd {
    pub(super) inner: ICmdShipChangeICtx = ICmdShipChangeICtx { .. },
}
impl ItemChangeShipCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.state = Some(state);
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.inner.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.inner.movement = Some(movement);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemChangeShipCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeShipCmd) -> Self {
        Self::Ship(sub_cmd)
    }
}
