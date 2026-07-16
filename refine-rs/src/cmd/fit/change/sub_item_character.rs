use crate::{
    ChangeFitEnumCmd,
    cmd::inner::{ICmdCharacterChangeICtx, ICmdCharacterSetICtx, ICmdCharacterUnsetICtx},
};

pub struct FitSetCharacterCmd {
    pub(super) inner: ICmdCharacterSetICtx,
}
impl FitSetCharacterCmd {
    pub fn new(type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdCharacterSetICtx { type_id, .. },
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
impl From<FitSetCharacterCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitSetCharacterCmd) -> Self {
        Self::SetCharacter(sub_cmd)
    }
}

#[derive(Default)]
pub struct FitChangeCharacterCmd {
    pub(super) inner: ICmdCharacterChangeICtx = ICmdCharacterChangeICtx { .. },
}
impl FitChangeCharacterCmd {
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
impl From<FitChangeCharacterCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeCharacterCmd) -> Self {
        Self::ChangeCharacter(sub_cmd)
    }
}

#[derive(Default)]
pub struct FitUnsetCharacterCmd {
    pub(super) inner: ICmdCharacterUnsetICtx = ICmdCharacterUnsetICtx,
}
impl FitUnsetCharacterCmd {
    pub fn new() -> Self {
        Self::default()
    }
}
impl From<FitUnsetCharacterCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitUnsetCharacterCmd) -> Self {
        Self::UnsetCharacter(sub_cmd)
    }
}
