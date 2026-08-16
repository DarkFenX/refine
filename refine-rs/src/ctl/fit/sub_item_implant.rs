use crate::{EffectId, EffectMode, FitCtlCmd, ItemIdBr, ItemTypeId, ctl::core::ICmdImplantChangeFCtxBIds};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitChangeImplantCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdImplantChangeFCtxBIds,
}
impl FitChangeImplantCmd {
    pub fn new(item_id: ItemIdBr) -> Self {
        Self {
            inner: ICmdImplantChangeFCtxBIds { item_id, .. },
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
impl From<FitChangeImplantCmd> for FitCtlCmd {
    fn from(sub_cmd: FitChangeImplantCmd) -> Self {
        Self::ChangeImplant(sub_cmd)
    }
}
