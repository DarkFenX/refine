use crate::{ChangeItemEnumCmd, EffectId, EffectMode, ItemTypeId, cmd::inner::ICmdStanceChangeICtx};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct ItemChangeStanceCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdStanceChangeICtx = ICmdStanceChangeICtx { .. },
}
impl ItemChangeStanceCmd {
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
impl From<ItemChangeStanceCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeStanceCmd) -> Self {
        Self::Stance(sub_cmd)
    }
}
