use crate::{ChangeFitEnumCmd, EffectId, EffectMode, ItemIdBackref, ItemTypeId, cmd::inner::ICmdChargeChangeFCtxBIds};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitChangeChargeCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdChargeChangeFCtxBIds,
}
impl FitChangeChargeCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdChargeChangeFCtxBIds { item_id, .. },
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
impl From<FitChangeChargeCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeChargeCmd) -> Self {
        Self::ChangeCharge(sub_cmd)
    }
}
