use crate::{
    AddItemEnumCmd, EffectId, EffectMode, FitId, ItemTypeId, ServiceState,
    cmd::inner::{ICmdServiceAddFCtxRIds, ICmdServiceAddICtx},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ItemAddServiceCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdServiceAddFCtxRIds,
}
impl ItemAddServiceCmd {
    pub fn new(fit_id: FitId, type_id: ItemTypeId, state: ServiceState) -> Self {
        Self {
            inner: ICmdServiceAddFCtxRIds {
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
impl From<ItemAddServiceCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddServiceCmd) -> Self {
        Self::Service(sub_cmd)
    }
}
