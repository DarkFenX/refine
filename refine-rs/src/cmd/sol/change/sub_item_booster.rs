use crate::{
    ChangeSolEnumCmd, EffectId, EffectMode, FitIdBackref, ItemIdBackref, ItemTypeId,
    cmd::inner::{ICmdBoosterAddFCtxBIds, ICmdBoosterAddICtx, ICmdBoosterChangeFCtxBIds},
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
    pub fn new(fit_id: FitIdBackref, type_id: ItemTypeId) -> Self {
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
impl From<SolAddBoosterCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddBoosterCmd) -> Self {
        Self::AddBooster(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeBoosterCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdBoosterChangeFCtxBIds,
}
impl SolChangeBoosterCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdBoosterChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
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
impl From<SolChangeBoosterCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeBoosterCmd) -> Self {
        Self::ChangeBooster(sub_cmd)
    }
}
