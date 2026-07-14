use crate::cmd::{
    ChangeSolEnumCmd,
    inner::{ICmdRigAddFCtxBIds, ICmdRigAddICtx},
    shared::FitIdBackref,
};

pub struct SolAddRigCmd {
    pub(super) inner: ICmdRigAddFCtxBIds,
}
impl SolAddRigCmd {
    pub fn new(fit_id: FitIdBackref, type_id: rc::ItemTypeId) -> Self {
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
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
