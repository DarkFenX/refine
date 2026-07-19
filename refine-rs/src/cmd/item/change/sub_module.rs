use crate::{
    ChangeItemEnumCmd, ChangeMutation, EffectId, EffectMode, ItemId, ItemTypeId, ModuleState, MoveMode, OptionalReload,
    Spool, cmd::inner::ICmdModuleChangeICtxRIds,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct ItemChangeModuleCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdModuleChangeICtxRIds = ICmdModuleChangeICtxRIds { .. },
}
impl ItemChangeModuleCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.shared.type_id = Some(type_id);
        self
    }
    pub fn with_move(mut self, move_: MoveMode) -> Self {
        self.inner.shared.move_ = Some(move_);
        self
    }
    pub fn with_state(mut self, state: ModuleState) -> Self {
        self.inner.shared.state = Some(state);
        self
    }
    pub fn with_mutation(mut self, mutation: Option<ChangeMutation>) -> Self {
        self.inner.shared.mutation = mutation.into();
        self
    }
    pub fn with_charge_type_id(mut self, charge_type_id: Option<ItemTypeId>) -> Self {
        self.inner.shared.charge_type_id = charge_type_id.into();
        self
    }
    pub fn with_spool(mut self, spool: Option<Spool>) -> Self {
        self.inner.shared.spool = spool.into();
        self
    }
    pub fn with_optional_reload(mut self, optional_reload: Option<OptionalReload>) -> Self {
        self.inner.shared.optional_reload = optional_reload.into();
        self
    }
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.inner.add_proj_item_ids.clear();
        self.inner.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.inner.rm_proj_item_ids.clear();
        self.inner.rm_proj_item_ids.extend(rm_proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.shared.effect_modes.clear();
        self.inner.shared.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemChangeModuleCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeModuleCmd) -> Self {
        Self::Module(sub_cmd)
    }
}
