use crate::{
    AddItemEnumCmd,
    cmd::inner::{ICmdImplantAddFCtxRIds, ICmdImplantAddICtx},
};

pub struct ItemAddImplantCmd {
    pub(super) inner: ICmdImplantAddFCtxRIds,
}
impl ItemAddImplantCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdImplantAddFCtxRIds {
                fit_id,
                ictx_cmd: ICmdImplantAddICtx { type_id, .. },
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
impl From<ItemAddImplantCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddImplantCmd) -> Self {
        Self::Implant(sub_cmd)
    }
}
