use crate::cmd::{ChangeFitEnumCmd, inner::ICmdRigAddICtx};

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
