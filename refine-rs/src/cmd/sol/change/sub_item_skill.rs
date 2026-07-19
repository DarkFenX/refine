use crate::{
    ChangeSolEnumCmd, EffectId, EffectMode, FitIdBackref, ItemIdBackref, ItemTypeId, SkillLevel,
    cmd::inner::{ICmdSkillAddFCtxBIds, ICmdSkillAddICtx, ICmdSkillChangeFCtxBIds},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolAddSkillCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdSkillAddFCtxBIds,
}
impl SolAddSkillCmd {
    pub fn new(fit_id: FitIdBackref, type_id: ItemTypeId, level: SkillLevel) -> Self {
        Self {
            inner: ICmdSkillAddFCtxBIds {
                fit_id,
                ictx_cmd: ICmdSkillAddICtx { type_id, level, .. },
            },
        }
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
impl From<SolAddSkillCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddSkillCmd) -> Self {
        Self::AddSkill(sub_cmd)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeSkillCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdSkillChangeFCtxBIds,
}
impl SolChangeSkillCmd {
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
impl From<SolChangeSkillCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeSkillCmd) -> Self {
        Self::ChangeSkill(sub_cmd)
    }
}
