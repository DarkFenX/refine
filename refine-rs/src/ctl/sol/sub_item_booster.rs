use crate::{
    EffectId, EffectMode, FitIdBr, ItemTypeId, SolCtlCmd,
    ctl::core::{ICmdBoosterAddFCtxBIds, ICmdBoosterAddICtx},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Add
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolAddBoosterCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdBoosterAddFCtxBIds,
}
impl SolAddBoosterCmd {
    pub fn new(fit_id: FitIdBr, type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdBoosterAddFCtxBIds {
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
impl From<SolAddBoosterCmd> for SolCtlCmd {
    fn from(sub_cmd: SolAddBoosterCmd) -> Self {
        Self::AddBooster(sub_cmd)
    }
}
