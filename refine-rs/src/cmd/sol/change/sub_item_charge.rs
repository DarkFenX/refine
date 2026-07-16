use crate::{ChangeSolEnumCmd, ItemIdBackref, cmd::inner::ICmdChargeChangeFCtxBIds};

pub struct SolChangeChargeCmd {
    pub(super) inner: ICmdChargeChangeFCtxBIds,
}
impl SolChangeChargeCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdChargeChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
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
impl From<SolChangeChargeCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeChargeCmd) -> Self {
        Self::ChangeCharge(sub_cmd)
    }
}
