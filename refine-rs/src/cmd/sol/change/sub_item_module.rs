use crate::cmd::{
    ChangeSolEnumCmd,
    inner::{ICmdModuleAddFCtxBIds, ICmdModuleAddICtxBIds, ICmdModuleAddShared, ICmdModuleChangeFCtxBIds},
    shared::{AddMutation, ChangeMutation, FitIdBackref, ItemIdBackref},
};

pub struct SolAddModuleCmd {
    pub(super) inner: ICmdModuleAddFCtxBIds,
}
impl SolAddModuleCmd {
    pub fn new(
        fit_id: FitIdBackref,
        rack: rc::ModRack,
        add_mode: rc::AddMode,
        type_id: rc::ItemTypeId,
        state: rc::ModuleState,
    ) -> Self {
        Self {
            inner: ICmdModuleAddFCtxBIds {
                fit_id,
                ictx_cmd: ICmdModuleAddICtxBIds {
                    shared: ICmdModuleAddShared {
                        rack,
                        add_mode,
                        type_id,
                        state,
                        ..
                    },
                    ..
                },
            },
        }
    }
    pub fn with_mutation(mut self, mutation: AddMutation) -> Self {
        self.inner.ictx_cmd.shared.mutation = Some(mutation);
        self
    }
    pub fn with_charge_type_id(mut self, charge_type_id: rc::ItemTypeId) -> Self {
        self.inner.ictx_cmd.shared.charge_type_id = Some(charge_type_id);
        self
    }
    pub fn with_spool(mut self, spool: rc::Spool) -> Self {
        self.inner.ictx_cmd.shared.spool = Some(spool);
        self
    }
    pub fn with_optional_reload(mut self, optional_reload: rc::OptionalReload) -> Self {
        self.inner.ictx_cmd.shared.optional_reload = Some(optional_reload);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemIdBackref>) -> Self {
        self.inner.ictx_cmd.proj_item_ids.clear();
        self.inner.ictx_cmd.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.shared.effect_modes.clear();
        self.inner.ictx_cmd.shared.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolAddModuleCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddModuleCmd) -> Self {
        Self::AddModule(sub_cmd)
    }
}

pub struct SolChangeModuleCmd {
    pub(super) inner: ICmdModuleChangeFCtxBIds,
}
impl SolChangeModuleCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdModuleChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.ictx_cmd.shared.type_id = Some(type_id);
        self
    }
    pub fn with_move(mut self, move_: rc::MoveMode) -> Self {
        self.inner.ictx_cmd.shared.move_ = Some(move_);
        self
    }
    pub fn with_state(mut self, state: rc::ModuleState) -> Self {
        self.inner.ictx_cmd.shared.state = Some(state);
        self
    }
    pub fn with_mutation(mut self, mutation: Option<ChangeMutation>) -> Self {
        self.inner.ictx_cmd.shared.mutation = mutation.into();
        self
    }
    pub fn with_charge_type_id(mut self, charge_type_id: Option<rc::ItemTypeId>) -> Self {
        self.inner.ictx_cmd.shared.charge_type_id = charge_type_id.into();
        self
    }
    pub fn with_spool(mut self, spool: Option<rc::Spool>) -> Self {
        self.inner.ictx_cmd.shared.spool = spool.into();
        self
    }
    pub fn with_optional_reload(mut self, optional_reload: Option<rc::OptionalReload>) -> Self {
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.shared.effect_modes.clear();
        self.inner.ictx_cmd.shared.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolChangeModuleCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeModuleCmd) -> Self {
        Self::ChangeModule(sub_cmd)
    }
}
