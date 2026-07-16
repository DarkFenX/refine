use crate::{
    AddItemEnumCmd, EffectId, EffectMode, FitId, ItemTypeId,
    cmd::inner::{ICmdBoosterAddFCtxRIds, ICmdBoosterAddICtx},
};

pub struct ItemAddBoosterCmd {
    pub(super) inner: ICmdBoosterAddFCtxRIds,
}
impl ItemAddBoosterCmd {
    pub fn new(fit_id: FitId, type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdBoosterAddFCtxRIds {
                fit_id,
                ictx_cmd: ICmdBoosterAddICtx { type_id, .. },
            },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_side_effects(mut self, side_effects: impl Iterator<Item = (EffectId, bool)>) -> Self {
        self.inner.ictx_cmd.side_effects.clear();
        self.inner.ictx_cmd.side_effects.extend(side_effects);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemAddBoosterCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddBoosterCmd) -> Self {
        Self::Booster(sub_cmd)
    }
}
