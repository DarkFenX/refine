use crate::{
    ChangeFitEnumCmd, EffectId, EffectMode, ItemIdBackref, ItemTypeId,
    cmd::inner::{ICmdRigAddICtx, ICmdRigChangeFCtxBIds},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Add
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitAddRigCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdRigAddICtx,
}
impl FitAddRigCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdRigAddICtx { type_id, .. },
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
impl From<FitAddRigCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitAddRigCmd) -> Self {
        Self::AddRig(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitChangeRigCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdRigChangeFCtxBIds,
}
impl FitChangeRigCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdRigChangeFCtxBIds { item_id, .. },
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
impl From<FitChangeRigCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeRigCmd) -> Self {
        Self::ChangeRig(sub_cmd)
    }
}
