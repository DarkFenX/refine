use crate::{
    AddMutation, AddedItemIdsResp, CmdResps, Coordinates, EffectId, EffectMode, FitId, FitIdBr, ItemId, ItemIdBr,
    ItemTypeId, MinionState, Movement, NpcProp, ctl::core::shared::EffectModes, err::BrResolveError,
    shared::CmdResidue,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct DroneAddCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    proj_item_ids: Vec<ItemId> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: DroneAddCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct DroneAddCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    proj_item_ids: Vec<ItemIdBr> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: DroneAddCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
struct DroneAddCmdShared {
    type_id: ItemTypeId,
    state: MinionState,
    mutation: Option<AddMutation> = None,
    npc_prop: Option<NpcProp> = None,
    coordinates: Option<Coordinates> = None,
    movement: Option<Movement> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct DroneAddCmdCtxFit {
    fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: DroneAddCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct DroneAddCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: DroneAddCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DroneAddCmd {
    pub fn new(type_id: ItemTypeId, state: MinionState) -> Self {
        Self {
            shared: DroneAddCmdShared { type_id, state, .. },
            ..
        }
    }
    pub fn with_mutation(mut self, mutation: AddMutation) -> Self {
        self.shared.mutation = Some(mutation);
        self
    }
    pub fn with_npc_prop(mut self, npc_prop: NpcProp) -> Self {
        self.shared.npc_prop = Some(npc_prop);
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
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.shared.effect_modes.extend(effect_modes);
        self
    }
}

impl DroneAddCmdBr {
    pub fn new(type_id: ItemTypeId, state: MinionState) -> Self {
        Self {
            shared: DroneAddCmdShared { type_id, state, .. },
            ..
        }
    }
    pub fn with_mutation(mut self, mutation: AddMutation) -> Self {
        self.shared.mutation = Some(mutation);
        self
    }
    pub fn with_npc_prop(mut self, npc_prop: NpcProp) -> Self {
        self.shared.npc_prop = Some(npc_prop);
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
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.proj_item_ids.extend(proj_item_ids);
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
impl DroneAddCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> DroneAddCmdCtxFit {
        DroneAddCmdCtxFit { fit_id, core: self }
    }
}

impl DroneAddCmdBr {
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> DroneAddCmdCtxFitBr {
        DroneAddCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DroneAddCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<DroneAddCmdCtxFit, BrResolveError> {
        Ok(DroneAddCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

impl DroneAddCmdBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<DroneAddCmd, BrResolveError> {
        Ok(DroneAddCmd {
            proj_item_ids: resps.resolve_item_ids(self.proj_item_ids)?,
            shared: self.shared,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DroneAddCmdBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self.proj_item_ids.is_empty() {
            true => CmdResidue::MutInfallible,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}
impl DroneAddCmdCtxFitBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self.core.proj_item_ids.is_empty() {
            true => CmdResidue::MutFallibleClean,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}

impl DroneAddCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, DroneAddError> {
        let mut core_drone = core_fit.add_drone(
            self.shared.type_id,
            self.shared.state,
            self.shared.coordinates,
            self.shared.movement,
        );
        if let Some(mutation) = self.shared.mutation.as_ref() {
            let mut core_mutation = core_drone.mutate(mutation.mutator_id).unwrap();
            mutation.apply_attrs(&mut core_mutation);
        }
        if let Some(npc_prop) = self.shared.npc_prop {
            core_drone.set_npc_prop(Some(npc_prop))
        }
        self.shared.effect_modes.apply(&mut core_drone);
        for projectee_item_id in self.proj_item_ids.iter() {
            core_drone.add_proj(projectee_item_id)?;
        }
        Ok(AddedItemIdsResp::from_core_drone(core_drone))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum DroneAddError {
    #[error("failed to add projection")]
    ProjAdd(#[from] rc::err::ProjAddError),
}

impl DroneAddCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, FitGetDroneAddError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetDroneAddError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
    #[error("failed to add projection")]
    ProjAdd(#[source] rc::err::ProjAddError),
}
impl From<DroneAddError> for FitGetDroneAddError {
    fn from(err: DroneAddError) -> Self {
        match err {
            DroneAddError::ProjAdd(inner) => Self::ProjAdd(inner),
        }
    }
}
