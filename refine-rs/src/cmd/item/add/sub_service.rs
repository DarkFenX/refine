use crate::cmd::{
    AddItemEnumCmd,
    inner::{ICmdServiceAddFCtxRIds, ICmdServiceAddICtx},
};

pub struct ItemAddServiceCmd {
    pub(super) inner: ICmdServiceAddFCtxRIds,
}
impl ItemAddServiceCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId, state: rc::ServiceState) -> Self {
        Self {
            inner: ICmdServiceAddFCtxRIds {
                fit_id,
                ictx_cmd: ICmdServiceAddICtx { type_id, state, .. },
            },
        }
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
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
