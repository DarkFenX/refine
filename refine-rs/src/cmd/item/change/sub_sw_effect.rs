use crate::{ChangeItemEnumCmd, EffectId, EffectMode, ItemTypeId, cmd::inner::ICmdSwEffectChangeICtx};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct ItemChangeSwEffectCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdSwEffectChangeICtx = ICmdSwEffectChangeICtx { .. },
}
impl ItemChangeSwEffectCmd {
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
impl From<ItemChangeSwEffectCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeSwEffectCmd) -> Self {
        Self::SwEffect(sub_cmd)
    }
}
