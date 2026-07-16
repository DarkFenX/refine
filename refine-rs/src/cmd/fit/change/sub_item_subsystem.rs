use crate::{
    ChangeFitEnumCmd, EffectId, EffectMode, ItemIdBackref, ItemTypeId,
    cmd::inner::{ICmdSubsystemAddICtx, ICmdSubsystemChangeFCtxBIds},
};

pub struct FitAddSubsystemCmd {
    pub(super) inner: ICmdSubsystemAddICtx,
}
impl FitAddSubsystemCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdSubsystemAddICtx { type_id, .. },
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
impl From<FitAddSubsystemCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitAddSubsystemCmd) -> Self {
        Self::AddSubsystem(sub_cmd)
    }
}

pub struct FitChangeSubsystemCmd {
    pub(super) inner: ICmdSubsystemChangeFCtxBIds,
}
impl FitChangeSubsystemCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdSubsystemChangeFCtxBIds { item_id, .. },
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitChangeSubsystemCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeSubsystemCmd) -> Self {
        Self::ChangeSubsystem(sub_cmd)
    }
}
