use crate::{
    ChangeSolEnumCmd, ItemIdBackref,
    cmd::inner::{ICmdProjEffectAddFCtxBIds, ICmdProjEffectAddShared, ICmdProjEffectChangeFCtxBIds},
};

pub struct SolAddProjEffectCmd {
    pub(super) inner: ICmdProjEffectAddFCtxBIds,
}
impl SolAddProjEffectCmd {
    pub fn new(type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdProjEffectAddFCtxBIds {
                shared: ICmdProjEffectAddShared { type_id, .. },
                ..
            },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.shared.state = Some(state);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemIdBackref>) -> Self {
        self.inner.proj_item_ids.clear();
        self.inner.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.shared.effect_modes.clear();
        self.inner.shared.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolAddProjEffectCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddProjEffectCmd) -> Self {
        Self::AddProjEffect(sub_cmd)
    }
}

pub struct SolChangeProjEffectCmd {
    pub(super) inner: ICmdProjEffectChangeFCtxBIds,
}
impl SolChangeProjEffectCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdProjEffectChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.ictx_cmd.shared.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.shared.state = Some(state);
        self
    }
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl Iterator<Item = ItemIdBackref>) -> Self {
        self.inner.ictx_cmd.add_proj_item_ids.clear();
        self.inner.ictx_cmd.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl Iterator<Item = ItemIdBackref>) -> Self {
        self.inner.ictx_cmd.rm_proj_item_ids.clear();
        self.inner.ictx_cmd.rm_proj_item_ids.extend(rm_proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.shared.effect_modes.clear();
        self.inner.ictx_cmd.shared.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolChangeProjEffectCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeProjEffectCmd) -> Self {
        Self::ChangeProjEffect(sub_cmd)
    }
}
