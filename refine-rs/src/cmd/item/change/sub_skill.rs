use crate::{ChangeItemEnumCmd, EffectId, EffectMode, ItemTypeId, SkillLevel, cmd::inner::ICmdSkillChangeICtx};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct ItemChangeSkillCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdSkillChangeICtx = ICmdSkillChangeICtx { .. },
}
impl ItemChangeSkillCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.type_id = Some(type_id);
        self
    }
    pub fn with_level(mut self, level: SkillLevel) -> Self {
        self.inner.level = Some(level);
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
impl From<ItemChangeSkillCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeSkillCmd) -> Self {
        Self::Skill(sub_cmd)
    }
}
