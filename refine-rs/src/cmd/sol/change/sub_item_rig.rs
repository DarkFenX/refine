use crate::{
    ChangeSolEnumCmd, EffectId, EffectMode, FitIdBackref, ItemIdBackref, ItemTypeId,
    cmd::inner::{ICmdRigAddFCtxBIds, ICmdRigAddICtx, ICmdRigChangeFCtxBIds},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolAddRigCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdRigAddFCtxBIds,
}
impl SolAddRigCmd {
    pub fn new(fit_id: FitIdBackref, type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdRigAddFCtxBIds {
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
impl From<SolAddRigCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddRigCmd) -> Self {
        Self::AddRig(sub_cmd)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeRigCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdRigChangeFCtxBIds,
}
impl SolChangeRigCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdRigChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
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
impl From<SolChangeRigCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeRigCmd) -> Self {
        Self::ChangeRig(sub_cmd)
    }
}
