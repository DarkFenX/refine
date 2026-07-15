use crate::cmd::{
    AddItemEnumCmd,
    inner::{ICmdShipSetFCtxRIds, ICmdShipSetICtx},
};

pub struct ItemSetShipCmd {
    pub(super) inner: ICmdShipSetFCtxRIds,
}
impl ItemSetShipCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdShipSetFCtxRIds {
                fit_id,
                ictx_cmd: ICmdShipSetICtx { type_id, .. },
            },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_coordinates(mut self, coordinates: rc::Coordinates) -> Self {
        self.inner.ictx_cmd.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: rc::Movement) -> Self {
        self.inner.ictx_cmd.movement = Some(movement);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemSetShipCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemSetShipCmd) -> Self {
        Self::Ship(sub_cmd)
    }
}
