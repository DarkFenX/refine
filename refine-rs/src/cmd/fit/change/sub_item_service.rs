use crate::{
    ChangeFitEnumCmd, EffectId, EffectMode, ItemIdBackref, ItemTypeId, ServiceState,
    cmd::inner::{ICmdServiceAddICtx, ICmdServiceChangeFCtxBIds},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Add
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitAddServiceCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdServiceAddICtx,
}
impl FitAddServiceCmd {
    pub fn new(type_id: ItemTypeId, state: ServiceState) -> Self {
        Self {
            inner: ICmdServiceAddICtx { type_id, state, .. },
        }
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitAddServiceCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitAddServiceCmd) -> Self {
        Self::AddService(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitChangeServiceCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdServiceChangeFCtxBIds,
}
impl FitChangeServiceCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdServiceChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: ServiceState) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitChangeServiceCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeServiceCmd) -> Self {
        Self::ChangeService(sub_cmd)
    }
}
