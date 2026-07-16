use crate::{ChangeItemEnumCmd, cmd::inner::ICmdBoosterChangeICtx};

#[derive(Default)]
pub struct ItemChangeBoosterCmd {
    pub(super) inner: ICmdBoosterChangeICtx = ICmdBoosterChangeICtx { .. },
}
impl ItemChangeBoosterCmd {
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
    pub fn with_side_effects(mut self, side_effects: impl Iterator<Item = (rc::EffectId, bool)>) -> Self {
        self.inner.side_effects.clear();
        self.inner.side_effects.extend(side_effects);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemChangeBoosterCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeBoosterCmd) -> Self {
        Self::Booster(sub_cmd)
    }
}
