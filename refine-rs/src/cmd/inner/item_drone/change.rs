use rc::ItemCommon;

use crate::{
    ChangeMutation, ChangedItemIdsResp, CmdResps, ItemIdBackref, TriStateField, cmd::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context
pub(in crate::cmd) struct ICmdDroneChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdDroneChangeICtxBIds = ICmdDroneChangeICtxBIds { .. },
}
pub(crate) struct ICmdDroneChangeFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: ICmdDroneChangeICtxRIds,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdDroneChangeICtxBIds {
    pub(in crate::cmd) shared: ICmdDroneChangeShared = ICmdDroneChangeShared { .. },
    pub(in crate::cmd) add_proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
    pub(in crate::cmd) rm_proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
}
pub(in crate::cmd) struct ICmdDroneChangeICtxRIds {
    pub(in crate::cmd) shared: ICmdDroneChangeShared = ICmdDroneChangeShared { .. },
    pub(in crate::cmd) add_proj_item_ids: Vec<rc::ItemId> = Vec::new(),
    pub(in crate::cmd) rm_proj_item_ids: Vec<rc::ItemId> = Vec::new(),
}
pub(in crate::cmd) struct ICmdDroneChangeShared {
    pub(in crate::cmd) type_id: Option<rc::ItemTypeId> = None,
    pub(in crate::cmd) state: Option<rc::MinionState> = None,
    pub(in crate::cmd) mutation: TriStateField<ChangeMutation> = TriStateField::Absent,
    pub(in crate::cmd) npc_prop: TriStateField<rc::NpcProp> = TriStateField::Absent,
    pub(in crate::cmd) coordinates: Option<rc::Coordinates> = None,
    pub(in crate::cmd) movement: Option<rc::Movement> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdDroneChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdDroneChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdDroneChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdDroneChangeICtxBIds {
    fn render(self, resps: &CmdResps) -> Result<ICmdDroneChangeICtxRIds, BackrefRenderError> {
        Ok(ICmdDroneChangeICtxRIds {
            shared: self.shared,
            add_proj_item_ids: resps.render_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.render_item_ids(self.rm_proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdDroneChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeDroneError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeDroneError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ItemChangeDroneError),
}

impl ICmdDroneChangeICtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeDroneError> {
        let core_drone = core_item.dc_drone()?;
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_drone.get_proj_mut(projectee_item_id)?.remove();
        }
        if let Some(type_id) = self.shared.type_id {
            core_drone.set_type_id(type_id);
        }
        if let Some(state) = self.shared.state {
            core_drone.set_state(state);
        }
        match &self.shared.mutation {
            TriStateField::Value(mutation) => {
                // Mutates item or updates existing mutation
                if let Some(mutator_id) = mutation.mutator_id {
                    match core_drone.get_mutation_mut() {
                        Some(core_mutation) => core_mutation.set_mutator_type_id(mutator_id),
                        None => core_drone.mutate(mutator_id).unwrap(),
                    };
                }
                if !mutation.attrs.is_empty() {
                    match core_drone.get_mutation_mut() {
                        Some(mut core_mutation) => mutation.apply_attrs(&mut core_mutation),
                        None => return Err(ItemChangeDroneError::NotMutated(core_drone.get_item_id())),
                    };
                }
            }
            TriStateField::None => {
                // Do nothing if mutation was not there
                if let Some(core_mutation) = core_drone.get_mutation_mut() {
                    core_mutation.remove();
                }
            }
            TriStateField::Absent => (),
        }
        match self.shared.npc_prop {
            TriStateField::Value(npc_prop) => core_drone.set_npc_prop(Some(npc_prop)),
            TriStateField::None => core_drone.set_npc_prop(None),
            TriStateField::Absent => (),
        }
        if let Some(coordinates) = self.shared.coordinates {
            core_drone.set_coordinates(coordinates);
        }
        if let Some(movement) = self.shared.movement {
            core_drone.set_movement(movement);
        }
        self.shared.effect_modes.apply(core_drone);
        for projectee_item_id in self.add_proj_item_ids.iter() {
            core_drone.add_proj(projectee_item_id)?;
        }
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeDroneError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
    #[error("unable to mutate attributes: item {0} is not mutated")]
    NotMutated(rc::ItemId),
    #[error("unable to add projection: {0}")]
    ProjAddFailed(#[from] rc::err::AddProjError),
    #[error("unable to remove projection: {0}")]
    ProjRemoveFailed(#[from] rc::err::GetRangedProjError),
}
