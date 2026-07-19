use crate::{
    AddMutation, ChangeMutation, ChangeSolEnumCmd, Coordinates, EffectId, EffectMode, FitIdBackref, ItemIdBackref,
    ItemTypeId, MinionState, Movement, NpcProp,
    cmd::inner::{ICmdDroneAddFCtxBIds, ICmdDroneAddICtxBIds, ICmdDroneAddShared, ICmdDroneChangeFCtxBIds},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolAddDroneCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdDroneAddFCtxBIds,
}
impl SolAddDroneCmd {
    pub fn new(fit_id: FitIdBackref, type_id: ItemTypeId, state: MinionState) -> Self {
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
    pub fn with_npc_prop(mut self, npc_prop: NpcProp) -> Self {
        self.inner.ictx_cmd.shared.npc_prop = Some(npc_prop);
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.inner.ictx_cmd.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.inner.ictx_cmd.shared.movement = Some(movement);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemIdBackref>) -> Self {
        self.inner.ictx_cmd.proj_item_ids.clear();
        self.inner.ictx_cmd.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
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

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeDroneCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdDroneChangeFCtxBIds,
}
impl SolChangeDroneCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdDroneChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.ictx_cmd.shared.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: MinionState) -> Self {
        self.inner.ictx_cmd.shared.state = Some(state);
        self
    }
    pub fn with_mutation(mut self, mutation: Option<ChangeMutation>) -> Self {
        self.inner.ictx_cmd.shared.mutation = mutation.into();
        self
    }
    pub fn with_npc_prop(mut self, npc_prop: Option<NpcProp>) -> Self {
        self.inner.ictx_cmd.shared.npc_prop = npc_prop.into();
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.inner.ictx_cmd.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
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
