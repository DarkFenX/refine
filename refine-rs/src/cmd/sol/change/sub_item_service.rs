use crate::{
    ChangeSolEnumCmd, FitIdBackref, ItemIdBackref,
    cmd::inner::{ICmdServiceAddFCtxBIds, ICmdServiceAddICtx, ICmdServiceChangeFCtxBIds},
};

pub struct SolAddServiceCmd {
    pub(super) inner: ICmdServiceAddFCtxBIds,
}
impl SolAddServiceCmd {
    pub fn new(fit_id: FitIdBackref, type_id: rc::ItemTypeId, state: rc::ServiceState) -> Self {
        Self {
            inner: ICmdServiceAddFCtxBIds {
                fit_id,
                ictx_cmd: ICmdServiceAddICtx { type_id, state, .. },
            },
        }
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolAddServiceCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddServiceCmd) -> Self {
        Self::AddService(sub_cmd)
    }
}

pub struct SolChangeServiceCmd {
    pub(super) inner: ICmdServiceChangeFCtxBIds,
}
impl SolChangeServiceCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdServiceChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: rc::ServiceState) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolChangeServiceCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeServiceCmd) -> Self {
        Self::ChangeService(sub_cmd)
    }
}
