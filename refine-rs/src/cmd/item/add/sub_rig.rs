use crate::{
    AddItemEnumCmd, EffectId, EffectMode, FitId, ItemTypeId,
    cmd::inner::{ICmdRigAddFCtxRIds, ICmdRigAddICtx},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ItemAddRigCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdRigAddFCtxRIds,
}
impl ItemAddRigCmd {
    pub fn new(fit_id: FitId, type_id: ItemTypeId) -> Self {
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
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
