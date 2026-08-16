use crate::{
    EffectId, EffectMode, FitIdBr, ItemIdBr, ItemTypeId, ServiceState, SolCtlCmd,
    ctl::core::{ICmdServiceAddFCtxBIds, ICmdServiceAddICtx, ICmdServiceChangeFCtxBIds},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Add
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolAddServiceCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdServiceAddFCtxBIds,
}
impl SolAddServiceCmd {
    pub fn new(fit_id: FitIdBr, type_id: ItemTypeId, state: ServiceState) -> Self {
        Self {
            inner: ICmdServiceAddFCtxBIds {
                fit_id,
                ictx_cmd: ICmdServiceAddICtx { type_id, state, .. },
            },
        }
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolAddServiceCmd> for SolCtlCmd {
    fn from(sub_cmd: SolAddServiceCmd) -> Self {
        Self::AddService(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeServiceCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdServiceChangeFCtxBIds,
}
impl SolChangeServiceCmd {
    pub fn new(item_id: ItemIdBr) -> Self {
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
impl From<SolChangeServiceCmd> for SolCtlCmd {
    fn from(sub_cmd: SolChangeServiceCmd) -> Self {
        Self::ChangeService(sub_cmd)
    }
}
