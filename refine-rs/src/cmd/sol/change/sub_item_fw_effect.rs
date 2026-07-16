use crate::{
    ChangeSolEnumCmd, EffectId, EffectMode, FitIdBackref, ItemIdBackref, ItemTypeId,
    cmd::inner::{ICmdFwEffectAddFCtxBIds, ICmdFwEffectAddICtx, ICmdFwEffectChangeFCtxBIds},
};

pub struct SolAddFwEffectCmd {
    pub(super) inner: ICmdFwEffectAddFCtxBIds,
}
impl SolAddFwEffectCmd {
    pub fn new(fit_id: FitIdBackref, type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdFwEffectAddFCtxBIds {
                fit_id,
                ictx_cmd: ICmdFwEffectAddICtx { type_id, .. },
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
impl From<SolAddFwEffectCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddFwEffectCmd) -> Self {
        Self::AddFwEffect(sub_cmd)
    }
}

pub struct SolChangeFwEffectCmd {
    pub(super) inner: ICmdFwEffectChangeFCtxBIds,
}
impl SolChangeFwEffectCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdFwEffectChangeFCtxBIds { item_id, .. },
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
impl From<SolChangeFwEffectCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeFwEffectCmd) -> Self {
        Self::ChangeFwEffect(sub_cmd)
    }
}
