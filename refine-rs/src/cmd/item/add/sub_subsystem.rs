use crate::{
    AddItemEnumCmd,
    cmd::inner::{ICmdSubsystemAddFCtxRIds, ICmdSubsystemAddICtx},
};

pub struct ItemAddSubsystemCmd {
    pub(super) inner: ICmdSubsystemAddFCtxRIds,
}
impl ItemAddSubsystemCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdSubsystemAddFCtxRIds {
                fit_id,
                ictx_cmd: ICmdSubsystemAddICtx { type_id, .. },
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
impl From<ItemAddSubsystemCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddSubsystemCmd) -> Self {
        Self::Subsystem(sub_cmd)
    }
}
