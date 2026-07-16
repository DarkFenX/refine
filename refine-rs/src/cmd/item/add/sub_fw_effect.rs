use crate::{
    AddItemEnumCmd,
    cmd::inner::{ICmdFwEffectAddFCtxRIds, ICmdFwEffectAddICtx},
};

pub struct ItemAddFwEffectCmd {
    pub(super) inner: ICmdFwEffectAddFCtxRIds,
}
impl ItemAddFwEffectCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdFwEffectAddFCtxRIds {
                fit_id,
                ictx_cmd: ICmdFwEffectAddICtx { type_id, .. },
            },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemAddFwEffectCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddFwEffectCmd) -> Self {
        Self::FwEffect(sub_cmd)
    }
}
