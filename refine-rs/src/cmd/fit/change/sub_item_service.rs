use crate::{
    ChangeFitEnumCmd, ItemIdBackref,
    cmd::inner::{ICmdServiceAddICtx, ICmdServiceChangeFCtxBIds},
};

pub struct FitAddServiceCmd {
    pub(super) inner: ICmdServiceAddICtx,
}
impl FitAddServiceCmd {
    pub fn new(type_id: rc::ItemTypeId, state: rc::ServiceState) -> Self {
        Self {
            inner: ICmdServiceAddICtx { type_id, state, .. },
        }
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitAddServiceCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitAddServiceCmd) -> Self {
        Self::AddService(sub_cmd)
    }
}

pub struct FitChangeServiceCmd {
    pub(super) inner: ICmdServiceChangeFCtxBIds,
}
impl FitChangeServiceCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdServiceChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: rc::ServiceState) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitChangeServiceCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeServiceCmd) -> Self {
        Self::ChangeService(sub_cmd)
    }
}
