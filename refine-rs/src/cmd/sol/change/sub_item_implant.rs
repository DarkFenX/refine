use crate::cmd::{
    ChangeSolEnumCmd,
    inner::{ICmdImplantAddFCtxBIds, ICmdImplantAddICtx, ICmdImplantChangeFCtxBIds},
    shared::{FitIdBackref, ItemIdBackref},
};

pub struct SolAddImplantCmd {
    pub(super) inner: ICmdImplantAddFCtxBIds,
}
impl SolAddImplantCmd {
    pub fn new(fit_id: FitIdBackref, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdImplantAddFCtxBIds {
                fit_id,
                ictx_cmd: ICmdImplantAddICtx { type_id, .. },
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
impl From<SolAddImplantCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddImplantCmd) -> Self {
        Self::AddImplant(sub_cmd)
    }
}

pub struct SolChangeImplantCmd {
    pub(super) inner: ICmdImplantChangeFCtxBIds,
}
impl SolChangeImplantCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdImplantChangeFCtxBIds { item_id, .. },
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
impl From<SolChangeImplantCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeImplantCmd) -> Self {
        Self::ChangeImplant(sub_cmd)
    }
}
