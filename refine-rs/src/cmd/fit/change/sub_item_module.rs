use crate::{
    AddMode, AddMutation, ChangeFitEnumCmd, ChangeMutation, EffectId, EffectMode, ItemIdBackref, ItemTypeId, ModRack,
    ModuleState, MoveMode, OptionalReload, Spool,
    cmd::inner::{ICmdModuleAddICtxBIds, ICmdModuleAddShared, ICmdModuleChangeFCtxBIds},
};

pub struct FitAddModuleCmd {
    pub(super) inner: ICmdModuleAddICtxBIds,
}
impl FitAddModuleCmd {
    pub fn new(rack: ModRack, add_mode: AddMode, type_id: ItemTypeId, state: ModuleState) -> Self {
        Self {
            inner: ICmdModuleAddICtxBIds {
                shared: ICmdModuleAddShared {
                    rack,
                    add_mode,
                    type_id,
                    state,
                    ..
                },
                ..
            },
        }
    }
    pub fn with_mutation(mut self, mutation: AddMutation) -> Self {
        self.inner.shared.mutation = Some(mutation);
        self
    }
    pub fn with_charge_type_id(mut self, charge_type_id: ItemTypeId) -> Self {
        self.inner.shared.charge_type_id = Some(charge_type_id);
        self
    }
    pub fn with_spool(mut self, spool: Spool) -> Self {
        self.inner.shared.spool = Some(spool);
        self
    }
    pub fn with_optional_reload(mut self, optional_reload: OptionalReload) -> Self {
        self.inner.shared.optional_reload = Some(optional_reload);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemIdBackref>) -> Self {
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
impl From<FitAddModuleCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitAddModuleCmd) -> Self {
        Self::AddModule(sub_cmd)
    }
}

pub struct FitChangeModuleCmd {
    pub(super) inner: ICmdModuleChangeFCtxBIds,
}
impl FitChangeModuleCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdModuleChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.ictx_cmd.shared.type_id = Some(type_id);
        self
    }
    pub fn with_move(mut self, move_: MoveMode) -> Self {
        self.inner.ictx_cmd.shared.move_ = Some(move_);
        self
    }
    pub fn with_state(mut self, state: ModuleState) -> Self {
        self.inner.ictx_cmd.shared.state = Some(state);
        self
    }
    pub fn with_mutation(mut self, mutation: Option<ChangeMutation>) -> Self {
        self.inner.ictx_cmd.shared.mutation = mutation.into();
        self
    }
    pub fn with_charge_type_id(mut self, charge_type_id: Option<ItemTypeId>) -> Self {
        self.inner.ictx_cmd.shared.charge_type_id = charge_type_id.into();
        self
    }
    pub fn with_spool(mut self, spool: Option<Spool>) -> Self {
        self.inner.ictx_cmd.shared.spool = spool.into();
        self
    }
    pub fn with_optional_reload(mut self, optional_reload: Option<OptionalReload>) -> Self {
        self.inner.ictx_cmd.shared.optional_reload = optional_reload.into();
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.shared.effect_modes.clear();
        self.inner.ictx_cmd.shared.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitChangeModuleCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeModuleCmd) -> Self {
        Self::ChangeModule(sub_cmd)
    }
}
