use crate::{ChangeItemEnumCmd, cmd::inner::ICmdCharacterChangeICtx};

#[derive(Default)]
pub struct ItemChangeCharacterCmd {
    pub(super) inner: ICmdCharacterChangeICtx = ICmdCharacterChangeICtx { .. },
}
impl ItemChangeCharacterCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.type_id = Some(type_id);
        self
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
impl From<ItemChangeCharacterCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeCharacterCmd) -> Self {
        Self::Character(sub_cmd)
    }
}
