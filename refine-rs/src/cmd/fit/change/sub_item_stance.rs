use crate::{
    ChangeFitEnumCmd, EffectId, EffectMode, ItemTypeId,
    cmd::inner::{ICmdStanceChangeICtx, ICmdStanceSetICtx, ICmdStanceUnsetICtx},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitSetStanceCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdStanceSetICtx,
}
impl FitSetStanceCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdStanceSetICtx { type_id, .. },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
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

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct FitChangeStanceCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdStanceChangeICtx = ICmdStanceChangeICtx { .. },
}
impl FitChangeStanceCmd {
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
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

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct FitUnsetStanceCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
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
