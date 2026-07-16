use crate::{ChangeSolEnumCmd, EffectId, EffectMode, ItemIdBackref, cmd::inner::ICmdAutochargeChangeFCtxBIds};

pub struct SolChangeAutochargeCmd {
    pub(super) inner: ICmdAutochargeChangeFCtxBIds,
}
impl SolChangeAutochargeCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdAutochargeChangeFCtxBIds { item_id, .. },
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
impl From<SolChangeAutochargeCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeAutochargeCmd) -> Self {
        Self::ChangeAutocharge(sub_cmd)
    }
}
