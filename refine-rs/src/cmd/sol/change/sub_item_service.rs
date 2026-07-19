use crate::{
    ChangeSolEnumCmd, EffectId, EffectMode, FitIdBackref, ItemIdBackref, ItemTypeId, ServiceState,
    cmd::inner::{ICmdServiceAddFCtxBIds, ICmdServiceAddICtx, ICmdServiceChangeFCtxBIds},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolAddServiceCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdServiceAddFCtxBIds,
}
impl SolAddServiceCmd {
    pub fn new(fit_id: FitIdBackref, type_id: ItemTypeId, state: ServiceState) -> Self {
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
impl From<SolAddServiceCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddServiceCmd) -> Self {
        Self::AddService(sub_cmd)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeServiceCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdServiceChangeFCtxBIds,
}
impl SolChangeServiceCmd {
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
impl From<SolChangeServiceCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeServiceCmd) -> Self {
        Self::ChangeService(sub_cmd)
    }
}
