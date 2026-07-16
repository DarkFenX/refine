use crate::{
    ChangeFitEnumCmd,
    cmd::inner::{ICmdStanceChangeICtx, ICmdStanceSetICtx, ICmdStanceUnsetICtx},
};

pub struct FitSetStanceCmd {
    pub(super) inner: ICmdStanceSetICtx,
}
impl FitSetStanceCmd {
    pub fn new(type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdStanceSetICtx { type_id, .. },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitSetStanceCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitSetStanceCmd) -> Self {
        Self::SetStance(sub_cmd)
    }
}

#[derive(Default)]
pub struct FitChangeStanceCmd {
    pub(super) inner: ICmdStanceChangeICtx = ICmdStanceChangeICtx { .. },
}
impl FitChangeStanceCmd {
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitChangeStanceCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeStanceCmd) -> Self {
        Self::ChangeStance(sub_cmd)
    }
}

#[derive(Default)]
pub struct FitUnsetStanceCmd {
    pub(super) inner: ICmdStanceUnsetICtx = ICmdStanceUnsetICtx,
}
impl FitUnsetStanceCmd {
    pub fn new() -> Self {
        Self::default()
    }
}
impl From<FitUnsetStanceCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitUnsetStanceCmd) -> Self {
        Self::UnsetStance(sub_cmd)
    }
}
