use crate::{AddItemEnumCmd, EffectId, EffectMode, ItemTypeId, cmd::inner::ICmdSwEffectAddFCtx};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ItemAddSwEffectCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdSwEffectAddFCtx,
}
impl ItemAddSwEffectCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdSwEffectAddFCtx { type_id, .. },
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
impl From<ItemAddSwEffectCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddSwEffectCmd) -> Self {
        Self::SwEffect(sub_cmd)
    }
}
