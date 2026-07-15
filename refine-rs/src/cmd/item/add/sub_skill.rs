use crate::cmd::{
    AddItemEnumCmd,
    inner::{ICmdSkillAddFCtxRIds, ICmdSkillAddICtx},
};

pub struct ItemAddSkillCmd {
    pub(super) inner: ICmdSkillAddFCtxRIds,
}
impl ItemAddSkillCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId, level: rc::SkillLevel) -> Self {
        Self {
            inner: ICmdSkillAddFCtxRIds {
                fit_id,
                ictx_cmd: ICmdSkillAddICtx { type_id, level, .. },
            },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemAddSkillCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddSkillCmd) -> Self {
        Self::Skill(sub_cmd)
    }
}
