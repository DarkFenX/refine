use crate::cmd::{
    ChangeFitEnumCmd,
    inner::{ICmdRigAddICtx, ICmdRigChangeFCtxBIds},
    shared::ItemIdBackref,
};

pub struct FitAddRigCmd {
    pub(super) inner: ICmdRigAddICtx,
}
impl FitAddRigCmd {
    pub fn new(type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdRigAddICtx { type_id, .. },
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
impl From<FitAddRigCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitAddRigCmd) -> Self {
        Self::AddRig(sub_cmd)
    }
}

pub struct FitChangeRigCmd {
    pub(super) inner: ICmdRigChangeFCtxBIds,
}
impl FitChangeRigCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdRigChangeFCtxBIds { item_id, .. },
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
impl From<FitChangeRigCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeRigCmd) -> Self {
        Self::ChangeRig(sub_cmd)
    }
}
