use crate::{
    ChangeFitEnumCmd, EffectId, EffectMode, ItemIdBackref, ItemTypeId,
    cmd::inner::{ICmdImplantAddICtx, ICmdImplantChangeFCtxBIds},
};

pub struct FitAddImplantCmd {
    pub(super) inner: ICmdImplantAddICtx,
}
impl FitAddImplantCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdImplantAddICtx { type_id, .. },
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
impl From<FitAddImplantCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitAddImplantCmd) -> Self {
        Self::AddImplant(sub_cmd)
    }
}

pub struct FitChangeImplantCmd {
    pub(super) inner: ICmdImplantChangeFCtxBIds,
}
impl FitChangeImplantCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdImplantChangeFCtxBIds { item_id, .. },
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
impl From<FitChangeImplantCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeImplantCmd) -> Self {
        Self::ChangeImplant(sub_cmd)
    }
}
