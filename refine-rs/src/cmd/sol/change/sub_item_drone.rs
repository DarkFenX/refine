use crate::cmd::{
    ChangeSolEnumCmd,
    inner::{ICmdDroneAddFCtxBIds, ICmdDroneAddICtxBIds, ICmdDroneAddShared, ICmdDroneChangeFCtxBIds},
    shared::{AddMutation, ChangeMutation, FitIdBackref, ItemIdBackref},
};

pub struct SolAddDroneCmd {
    pub(super) inner: ICmdDroneAddFCtxBIds,
}
impl SolAddDroneCmd {
    pub fn new(fit_id: FitIdBackref, type_id: rc::ItemTypeId, state: rc::MinionState) -> Self {
        Self {
            inner: ICmdDroneAddFCtxBIds {
                fit_id,
                ictx_cmd: ICmdDroneAddICtxBIds {
                    shared: ICmdDroneAddShared { type_id, state, .. },
                    ..
                },
            },
        }
    }
    pub fn with_mutation(mut self, mutation: AddMutation) -> Self {
        self.inner.ictx_cmd.shared.mutation = Some(mutation);
        self
    }
    pub fn with_npc_prop(mut self, npc_prop: rc::NpcProp) -> Self {
        self.inner.ictx_cmd.shared.npc_prop = Some(npc_prop);
        self
    }
    pub fn with_coordinates(mut self, coordinates: rc::Coordinates) -> Self {
        self.inner.ictx_cmd.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: rc::Movement) -> Self {
        self.inner.ictx_cmd.shared.movement = Some(movement);
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
impl From<SolAddDroneCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddDroneCmd) -> Self {
        Self::AddDrone(sub_cmd)
    }
}

pub struct SolChangeDroneCmd {
    pub(super) inner: ICmdDroneChangeFCtxBIds,
}
impl SolChangeDroneCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdDroneChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.ictx_cmd.shared.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: rc::MinionState) -> Self {
        self.inner.ictx_cmd.shared.state = Some(state);
        self
    }
    pub fn with_mutation(mut self, mutation: Option<ChangeMutation>) -> Self {
        self.inner.ictx_cmd.shared.mutation = mutation.into();
        self
    }
    pub fn with_npc_prop(mut self, npc_prop: Option<rc::NpcProp>) -> Self {
        self.inner.ictx_cmd.shared.npc_prop = npc_prop.into();
        self
    }
    pub fn with_coordinates(mut self, coordinates: rc::Coordinates) -> Self {
        self.inner.ictx_cmd.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: rc::Movement) -> Self {
        self.inner.ictx_cmd.shared.movement = Some(movement);
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
impl From<SolChangeDroneCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeDroneCmd) -> Self {
        Self::ChangeDrone(sub_cmd)
    }
}
