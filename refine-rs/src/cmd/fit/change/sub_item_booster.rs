use crate::{
    ChangeFitEnumCmd, EffectId, EffectMode, ItemIdBackref, ItemTypeId,
    cmd::inner::{ICmdBoosterAddICtx, ICmdBoosterChangeFCtxBIds},
};

pub struct FitAddBoosterCmd {
    pub(super) inner: ICmdBoosterAddICtx,
}
impl FitAddBoosterCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdBoosterAddICtx { type_id, .. },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.state = Some(state);
        self
    }
    pub fn with_side_effects(mut self, side_effects: impl Iterator<Item = (EffectId, bool)>) -> Self {
        self.inner.side_effects.clear();
        self.inner.side_effects.extend(side_effects);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitAddBoosterCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitAddBoosterCmd) -> Self {
        Self::AddBooster(sub_cmd)
    }
}

pub struct FitChangeBoosterCmd {
    pub(super) inner: ICmdBoosterChangeFCtxBIds,
}
impl FitChangeBoosterCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdBoosterChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_side_effects(mut self, side_effects: impl Iterator<Item = (EffectId, bool)>) -> Self {
        self.inner.ictx_cmd.side_effects.clear();
        self.inner.ictx_cmd.side_effects.extend(side_effects);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitChangeBoosterCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeBoosterCmd) -> Self {
        Self::ChangeBooster(sub_cmd)
    }
}
