use crate::{
    EffectId, EffectMode, FitCtlCmd, ItemTypeId,
    ctl::core::{ICmdCharacterChangeICtx, ICmdCharacterSetICtx},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Set
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitSetCharacterCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdCharacterSetICtx,
}
impl FitSetCharacterCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdCharacterSetICtx { type_id, .. },
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
impl From<FitSetCharacterCmd> for FitCtlCmd {
    fn from(sub_cmd: FitSetCharacterCmd) -> Self {
        Self::SetCharacter(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct FitChangeCharacterCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdCharacterChangeICtx = ICmdCharacterChangeICtx { .. },
}
impl FitChangeCharacterCmd {
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
impl From<FitChangeCharacterCmd> for FitCtlCmd {
    fn from(sub_cmd: FitChangeCharacterCmd) -> Self {
        Self::ChangeCharacter(sub_cmd)
    }
}
