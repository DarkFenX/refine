use crate::{EffectId, EffectMode, FitCtlCmd, ItemTypeId, ctl::core::ICmdBoosterAddICtx};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Add
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitAddBoosterCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdBoosterAddICtx,
}
impl FitAddBoosterCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdBoosterAddICtx { type_id, .. },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.state = Some(state);
        self
    }
    pub fn with_side_effects(mut self, side_effects: impl Iterator<Item = (EffectId, bool)>) -> Self {
        self.inner.side_effects.clear();
        self.inner.side_effects.extend(side_effects);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitAddBoosterCmd> for FitCtlCmd {
    fn from(sub_cmd: FitAddBoosterCmd) -> Self {
        Self::AddBooster(sub_cmd)
    }
}
