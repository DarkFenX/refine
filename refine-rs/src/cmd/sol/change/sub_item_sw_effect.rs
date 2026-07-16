use crate::{
    ChangeSolEnumCmd, EffectId, EffectMode, ItemIdBackref, ItemTypeId,
    cmd::inner::{ICmdSwEffectAddFCtx, ICmdSwEffectChangeFCtxBIds},
};

pub struct SolAddSwEffectCmd {
    pub(super) inner: ICmdSwEffectAddFCtx,
}
impl SolAddSwEffectCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdSwEffectAddFCtx { type_id, .. },
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
impl From<SolAddSwEffectCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddSwEffectCmd) -> Self {
        Self::AddSwEffect(sub_cmd)
    }
}

pub struct SolChangeSwEffectCmd {
    pub(super) inner: ICmdSwEffectChangeFCtxBIds,
}
impl SolChangeSwEffectCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdSwEffectChangeFCtxBIds { item_id, .. },
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
impl From<SolChangeSwEffectCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeSwEffectCmd) -> Self {
        Self::ChangeSwEffect(sub_cmd)
    }
}
