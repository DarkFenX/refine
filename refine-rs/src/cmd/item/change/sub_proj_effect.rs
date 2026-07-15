use crate::cmd::{ChangeItemEnumCmd, inner::ICmdProjEffectChangeICtxRIds};

#[derive(Default)]
pub struct ItemChangeProjEffectCmd {
    pub(super) inner: ICmdProjEffectChangeICtxRIds = ICmdProjEffectChangeICtxRIds { .. },
}
impl ItemChangeProjEffectCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.shared.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.shared.state = Some(state);
        self
    }
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl Iterator<Item = rc::ItemId>) -> Self {
        self.inner.add_proj_item_ids.clear();
        self.inner.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl Iterator<Item = rc::ItemId>) -> Self {
        self.inner.rm_proj_item_ids.clear();
        self.inner.rm_proj_item_ids.extend(rm_proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.shared.effect_modes.clear();
        self.inner.shared.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemChangeProjEffectCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeProjEffectCmd) -> Self {
        Self::ProjEffect(sub_cmd)
    }
}
