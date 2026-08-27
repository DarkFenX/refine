use rc::ItemCommon;

use crate::{
    ChangeMutation, ChangedItemIdsResp, CmdResps, Coordinates, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId,
    MinionState, Movement, NpcProp, TriStateField, ctl::core::shared::EffectModes, err::BrResolveError,
    shared::CmdResidue,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct DroneChangeCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    add_proj_item_ids: Vec<ItemId>,
    #[cfg_attr(feature = "serde", serde(default))]
    rm_proj_item_ids: Vec<ItemId>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: DroneChangeCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct DroneChangeCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    add_proj_item_ids: Vec<ItemIdBr>,
    #[cfg_attr(feature = "serde", serde(default))]
    rm_proj_item_ids: Vec<ItemIdBr>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: DroneChangeCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
struct DroneChangeCmdShared {
    type_id: Option<ItemTypeId>,
    state: Option<MinionState>,
    #[cfg_attr(feature = "serde", serde(default))]
    mutation: TriStateField<ChangeMutation>,
    #[cfg_attr(feature = "serde", serde(default))]
    npc_prop: TriStateField<NpcProp>,
    coordinates: Option<Coordinates>,
    movement: Option<Movement>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct DroneChangeCmdCtxItem {
    item_id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: DroneChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct DroneChangeCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: DroneChangeCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DroneChangeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.shared.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: MinionState) -> Self {
        self.shared.state = Some(state);
        self
    }
    pub fn with_mutation(mut self, mutation: Option<ChangeMutation>) -> Self {
        self.shared.mutation = mutation.into();
        self
    }
    pub fn with_npc_prop(mut self, npc_prop: Option<NpcProp>) -> Self {
        self.shared.npc_prop = npc_prop.into();
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.shared.movement = Some(movement);
        self
    }
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.rm_proj_item_ids.extend(rm_proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.shared.effect_modes.extend(effect_modes);
        self
    }
}

impl DroneChangeCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.shared.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: MinionState) -> Self {
        self.shared.state = Some(state);
        self
    }
    pub fn with_mutation(mut self, mutation: Option<ChangeMutation>) -> Self {
        self.shared.mutation = mutation.into();
        self
    }
    pub fn with_npc_prop(mut self, npc_prop: Option<NpcProp>) -> Self {
        self.shared.npc_prop = npc_prop.into();
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.shared.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.shared.movement = Some(movement);
        self
    }
    pub fn with_add_proj_item_ids(mut self, add_proj_item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.add_proj_item_ids.extend(add_proj_item_ids);
        self
    }
    pub fn with_rm_proj_item_ids(mut self, rm_proj_item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.rm_proj_item_ids.extend(rm_proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.shared.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DroneChangeCmd {
    pub(in crate::ctl) fn into_ctx_item(self, item_id: ItemId) -> DroneChangeCmdCtxItem {
        DroneChangeCmdCtxItem { item_id, core: self }
    }
}

impl DroneChangeCmdBr {
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> DroneChangeCmdCtxItemBr {
        DroneChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DroneChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<DroneChangeCmdCtxItem, BrResolveError> {
        Ok(DroneChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

impl DroneChangeCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<DroneChangeCmd, BrResolveError> {
        Ok(DroneChangeCmd {
            add_proj_item_ids: resps.resolve_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.resolve_item_ids(self.rm_proj_item_ids)?,
            shared: self.shared,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DroneChangeCmdBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        // Assume the command always mutates (even if it does not with none of fields set)
        if !self.rm_proj_item_ids.is_empty() || !self.add_proj_item_ids.is_empty() {
            return CmdResidue::MutFallibleDirty;
        }
        if let TriStateField::Value(mutation) = &self.shared.mutation
            && !mutation.attrs.is_empty()
        {
            return CmdResidue::MutFallibleDirty;
        }
        CmdResidue::MutFallibleClean
    }
}
impl DroneChangeCmdCtxItemBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        self.core.exec_residue()
    }
}

impl DroneChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, DroneChangeError> {
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
                        None => return Err(DroneChangeError::NotMutated(core_drone.get_item_id())),
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
pub enum DroneChangeError {
    #[error(transparent)]
    ItemIsNotDrone(#[from] rc::err::ItemKindMatchError),
    #[error("unable to mutate attributes: item {0} is not mutated")]
    NotMutated(ItemId),
    #[error("unable to add projection")]
    ProjAdd(#[from] rc::err::ProjAddError),
    #[error("unable to remove projection")]
    ProjRemove(#[from] rc::err::ProjGetError),
}

impl DroneChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetDroneChangeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetDroneChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::ItemGetError),
    #[error(transparent)]
    ItemIsNotDrone(rc::err::ItemKindMatchError),
    #[error("unable to mutate attributes: item {0} is not mutated")]
    NotMutated(ItemId),
    #[error("unable to add projection")]
    ProjAdd(#[source] rc::err::ProjAddError),
    #[error("unable to remove projection")]
    ProjRemove(#[source] rc::err::ProjGetError),
}
impl From<DroneChangeError> for ItemGetDroneChangeError {
    fn from(err: DroneChangeError) -> Self {
        match err {
            DroneChangeError::ItemIsNotDrone(inner) => Self::ItemIsNotDrone(inner),
            DroneChangeError::NotMutated(inner) => Self::NotMutated(inner),
            DroneChangeError::ProjAdd(inner) => Self::ProjAdd(inner),
            DroneChangeError::ProjRemove(inner) => Self::ProjRemove(inner),
        }
    }
}
