use crate::{ChangeItemEnumCmd, EffectId, EffectMode, ItemTypeId, ServiceState, cmd::inner::ICmdServiceChangeICtx};

#[derive(Default)]
pub struct ItemChangeServiceCmd {
    pub(super) inner: ICmdServiceChangeICtx = ICmdServiceChangeICtx { .. },
}
impl ItemChangeServiceCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: ServiceState) -> Self {
        self.inner.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemChangeServiceCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeServiceCmd) -> Self {
        Self::Service(sub_cmd)
    }
}
