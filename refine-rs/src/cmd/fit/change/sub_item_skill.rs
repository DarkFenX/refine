use crate::{
    ChangeFitEnumCmd, EffectId, EffectMode, ItemIdBackref, ItemTypeId, SkillLevel,
    cmd::inner::{ICmdSkillAddICtx, ICmdSkillChangeFCtxBIds},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Add
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitAddSkillCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdSkillAddICtx,
}
impl FitAddSkillCmd {
    pub fn new(type_id: ItemTypeId, level: SkillLevel) -> Self {
        Self {
            inner: ICmdSkillAddICtx { type_id, level, .. },
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
impl From<FitAddSkillCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitAddSkillCmd) -> Self {
        Self::AddSkill(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitChangeSkillCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdSkillChangeFCtxBIds,
}
impl FitChangeSkillCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdSkillChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
    }
    pub fn with_level(mut self, level: SkillLevel) -> Self {
        self.inner.ictx_cmd.level = Some(level);
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
impl From<FitChangeSkillCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeSkillCmd) -> Self {
        Self::ChangeSkill(sub_cmd)
    }
}
