use crate::{
    AddItemEnumCmd, EffectId, EffectMode, ItemId, ItemTypeId,
    cmd::inner::{ICmdProjEffectAddFCtxRIds, ICmdProjEffectAddShared},
};

pub struct ItemAddProjEffectCmd {
    pub(super) inner: ICmdProjEffectAddFCtxRIds,
}
impl ItemAddProjEffectCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self {
            inner: ICmdProjEffectAddFCtxRIds {
                shared: ICmdProjEffectAddShared { type_id, .. },
                ..
            },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.shared.state = Some(state);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.inner.proj_item_ids.clear();
        self.inner.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.shared.effect_modes.clear();
        self.inner.shared.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemAddProjEffectCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddProjEffectCmd) -> Self {
        Self::ProjEffect(sub_cmd)
    }
}
