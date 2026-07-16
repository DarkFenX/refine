use crate::{ChangeItemEnumCmd, EffectId, EffectMode, ItemTypeId, cmd::inner::ICmdChargeChangeICtx};

#[derive(Default)]
pub struct ItemChangeChargeCmd {
    pub(super) inner: ICmdChargeChangeICtx = ICmdChargeChangeICtx { .. },
}
impl ItemChangeChargeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.type_id = Some(type_id);
        self
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
impl From<ItemChangeChargeCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeChargeCmd) -> Self {
        Self::Charge(sub_cmd)
    }
}
