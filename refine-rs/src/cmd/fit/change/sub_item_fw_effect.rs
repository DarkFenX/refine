use crate::cmd::{
    ChangeFitEnumCmd,
    inner::{ICmdFwEffectAddICtx, ICmdFwEffectChangeFCtxBIds},
    shared::ItemIdBackref,
};

pub struct FitAddFwEffectCmd {
    pub(super) inner: ICmdFwEffectAddICtx,
}
impl FitAddFwEffectCmd {
    pub fn new(type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdFwEffectAddICtx { type_id, .. },
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
impl From<FitAddFwEffectCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitAddFwEffectCmd) -> Self {
        Self::AddFwEffect(sub_cmd)
    }
}

pub struct FitChangeFwEffectCmd {
    pub(super) inner: ICmdFwEffectChangeFCtxBIds,
}
impl FitChangeFwEffectCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdFwEffectChangeFCtxBIds { item_id, .. },
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
impl From<FitChangeFwEffectCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeFwEffectCmd) -> Self {
        Self::ChangeFwEffect(sub_cmd)
    }
}
