use crate::{
    AddItemEnumCmd,
    cmd::inner::{ICmdRigAddFCtxRIds, ICmdRigAddICtx},
};

pub struct ItemAddRigCmd {
    pub(super) inner: ICmdRigAddFCtxRIds,
}
impl ItemAddRigCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdRigAddFCtxRIds {
                fit_id,
                ictx_cmd: ICmdRigAddICtx { type_id, .. },
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
impl From<ItemAddRigCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddRigCmd) -> Self {
        Self::Rig(sub_cmd)
    }
}
