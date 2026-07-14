use crate::cmd::{
    AddItemEnumCmd,
    inner::{ICmdCharacterSetFCtxRIds, ICmdCharacterSetICtx},
};

pub struct ItemSetCharacterCmd {
    pub(super) inner: ICmdCharacterSetFCtxRIds,
}
impl ItemSetCharacterCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdCharacterSetFCtxRIds {
                fit_id,
                ictx_cmd: ICmdCharacterSetICtx { type_id, .. },
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
impl From<ItemSetCharacterCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemSetCharacterCmd) -> Self {
        Self::Character(sub_cmd)
    }
}
