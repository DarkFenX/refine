use crate::{
    ChangeSolEnumCmd, EffectId, EffectMode, FitIdBackref, ItemIdBackref, ItemTypeId,
    cmd::inner::{ICmdSubsystemAddFCtxBIds, ICmdSubsystemAddICtx, ICmdSubsystemChangeFCtxBIds},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Add
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolAddSubsystemCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdSubsystemAddFCtxBIds,
}
impl SolAddSubsystemCmd {
    pub fn new(fit_id: FitIdBackref, type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdSubsystemAddFCtxBIds {
                fit_id,
                ictx_cmd: ICmdSubsystemAddICtx { type_id, .. },
            },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolAddSubsystemCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddSubsystemCmd) -> Self {
        Self::AddSubsystem(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeSubsystemCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdSubsystemChangeFCtxBIds,
}
impl SolChangeSubsystemCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdSubsystemChangeFCtxBIds { item_id, .. },
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolChangeSubsystemCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeSubsystemCmd) -> Self {
        Self::ChangeSubsystem(sub_cmd)
    }
}
