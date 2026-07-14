use crate::cmd::{ChangeItemEnumCmd, inner::ICmdAutochargeChangeICtx};

#[derive(Default)]
pub struct ItemChangeAutochargeCmd {
    pub(super) inner: ICmdAutochargeChangeICtx = ICmdAutochargeChangeICtx { .. },
}
impl ItemChangeAutochargeCmd {
    pub fn new() -> Self {
        Self::default()
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
impl From<ItemChangeAutochargeCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeAutochargeCmd) -> Self {
        Self::Autocharge(sub_cmd)
    }
}
