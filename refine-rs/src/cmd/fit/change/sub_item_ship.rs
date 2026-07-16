use crate::{
    ChangeFitEnumCmd,
    cmd::inner::{ICmdShipChangeICtx, ICmdShipSetICtx, ICmdShipUnsetICtx},
};

pub struct FitSetShipCmd {
    pub(super) inner: ICmdShipSetICtx,
}
impl FitSetShipCmd {
    pub fn new(type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdShipSetICtx { type_id, .. },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.state = Some(state);
        self
    }
    pub fn with_coordinates(mut self, coordinates: rc::Coordinates) -> Self {
        self.inner.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: rc::Movement) -> Self {
        self.inner.movement = Some(movement);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitSetShipCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitSetShipCmd) -> Self {
        Self::SetShip(sub_cmd)
    }
}

#[derive(Default)]
pub struct FitChangeShipCmd {
    pub(super) inner: ICmdShipChangeICtx = ICmdShipChangeICtx { .. },
}
impl FitChangeShipCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.state = Some(state);
        self
    }
    pub fn with_coordinates(mut self, coordinates: rc::Coordinates) -> Self {
        self.inner.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: rc::Movement) -> Self {
        self.inner.movement = Some(movement);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitChangeShipCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeShipCmd) -> Self {
        Self::ChangeShip(sub_cmd)
    }
}

#[derive(Default)]
pub struct FitUnsetShipCmd {
    pub(super) inner: ICmdShipUnsetICtx = ICmdShipUnsetICtx,
}
impl FitUnsetShipCmd {
    pub fn new() -> Self {
        Self::default()
    }
}
impl From<FitUnsetShipCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitUnsetShipCmd) -> Self {
        Self::UnsetShip(sub_cmd)
    }
}
