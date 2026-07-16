use crate::{
    AddItemEnumCmd, AddMutation,
    cmd::inner::{ICmdModuleAddFCtxRIds, ICmdModuleAddICtxRIds, ICmdModuleAddShared},
};

pub struct ItemAddModuleCmd {
    pub(super) inner: ICmdModuleAddFCtxRIds,
}
impl ItemAddModuleCmd {
    pub fn new(
        fit_id: rc::FitId,
        rack: rc::ModRack,
        add_mode: rc::AddMode,
        type_id: rc::ItemTypeId,
        state: rc::ModuleState,
    ) -> Self {
        Self {
            inner: ICmdModuleAddFCtxRIds {
                fit_id,
                ictx_cmd: ICmdModuleAddICtxRIds {
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
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = rc::ItemId>) -> Self {
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
impl From<ItemAddModuleCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddModuleCmd) -> Self {
        Self::Module(sub_cmd)
    }
}
