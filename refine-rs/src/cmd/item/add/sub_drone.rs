use crate::{
    AddItemEnumCmd, AddMutation,
    cmd::inner::{ICmdDroneAddFCtxRIds, ICmdDroneAddICtxRIds, ICmdDroneAddShared},
};

pub struct ItemAddDroneCmd {
    pub(super) inner: ICmdDroneAddFCtxRIds,
}
impl ItemAddDroneCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId, state: rc::MinionState) -> Self {
        Self {
            inner: ICmdDroneAddFCtxRIds {
                fit_id,
                ictx_cmd: ICmdDroneAddICtxRIds {
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
impl From<ItemAddDroneCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddDroneCmd) -> Self {
        Self::Drone(sub_cmd)
    }
}
