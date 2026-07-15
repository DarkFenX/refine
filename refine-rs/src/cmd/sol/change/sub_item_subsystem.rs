use crate::cmd::{
    ChangeSolEnumCmd,
    inner::{ICmdSubsystemAddFCtxBIds, ICmdSubsystemAddICtx, ICmdSubsystemChangeFCtxBIds},
    shared::{FitIdBackref, ItemIdBackref},
};

pub struct SolAddSubsystemCmd {
    pub(super) inner: ICmdSubsystemAddFCtxBIds,
}
impl SolAddSubsystemCmd {
    pub fn new(fit_id: FitIdBackref, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdSubsystemAddFCtxBIds {
                fit_id,
                ictx_cmd: ICmdSubsystemAddICtx { type_id, .. },
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
impl From<SolAddSubsystemCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddSubsystemCmd) -> Self {
        Self::AddSubsystem(sub_cmd)
    }
}

pub struct SolChangeSubsystemCmd {
    pub(super) inner: ICmdSubsystemChangeFCtxBIds,
}
impl SolChangeSubsystemCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdSubsystemChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
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
impl From<SolChangeSubsystemCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeSubsystemCmd) -> Self {
        Self::ChangeSubsystem(sub_cmd)
    }
}
