use crate::{ChangeFitEnumCmd, ItemIdBackref, cmd::inner::ICmdAutochargeChangeFCtxBIds};

pub struct FitChangeAutochargeCmd {
    pub(super) inner: ICmdAutochargeChangeFCtxBIds,
}
impl FitChangeAutochargeCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdAutochargeChangeFCtxBIds { item_id, .. },
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
impl From<FitChangeAutochargeCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeAutochargeCmd) -> Self {
        Self::ChangeAutocharge(sub_cmd)
    }
}
