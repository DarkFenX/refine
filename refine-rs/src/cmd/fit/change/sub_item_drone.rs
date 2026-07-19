use crate::{
    AddMutation, ChangeFitEnumCmd, ChangeMutation, Coordinates, EffectId, EffectMode, ItemIdBackref, ItemTypeId,
    MinionState, Movement, NpcProp,
    cmd::inner::{ICmdDroneAddICtxBIds, ICmdDroneAddShared, ICmdDroneChangeFCtxBIds},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Add
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitAddDroneCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdDroneAddICtxBIds,
}
impl FitAddDroneCmd {
    pub fn new(type_id: ItemTypeId, state: MinionState) -> Self {
        Self {
            inner: ICmdDroneAddICtxBIds {
                shared: ICmdDroneAddShared { type_id, state, .. },
                ..
            },
        }
    }
    pub fn with_mutation(mut self, mutation: AddMutation) -> Self {
        self.inner.shared.mutation = Some(mutation);
        self
    }
    pub fn with_npc_prop(mut self, npc_prop: NpcProp) -> Self {
        self.inner.shared.npc_prop = Some(npc_prop);
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.inner.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.inner.shared.movement = Some(movement);
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
impl From<FitAddDroneCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitAddDroneCmd) -> Self {
        Self::AddDrone(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitChangeDroneCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdDroneChangeFCtxBIds,
}
impl FitChangeDroneCmd {
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
impl From<FitChangeDroneCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeDroneCmd) -> Self {
        Self::ChangeDrone(sub_cmd)
    }
}
