use crate::{AddItemEnumCmd, cmd::inner::ICmdSwEffectAddFCtx};

pub struct ItemAddSwEffectCmd {
    pub(super) inner: ICmdSwEffectAddFCtx,
}
impl ItemAddSwEffectCmd {
    pub fn new(type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdSwEffectAddFCtx { type_id, .. },
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
impl From<ItemAddSwEffectCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddSwEffectCmd) -> Self {
        Self::SwEffect(sub_cmd)
    }
}
