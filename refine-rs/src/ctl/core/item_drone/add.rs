use crate::{
    AddMutation, AddedItemIdsResp, CmdResps, Coordinates, EffectId, EffectMode, FitId, FitIdBr, ItemId, ItemIdBr,
    ItemTypeId, MinionState, Movement, NpcProp, ctl::core::shared::EffectModes, err::BrResolveError,
    shared::CmdResidue,
};

// Core commands
pub type DroneAddCmd = DroneAddCmdGen<ItemId>;
pub type DroneAddCmdBr = DroneAddCmdGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct DroneAddCmdGen<I> {
    type_id: ItemTypeId,
    state: MinionState,
    mutation: Option<AddMutation> = None,
    npc_prop: Option<NpcProp> = None,
    coordinates: Option<Coordinates> = None,
    movement: Option<Movement> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    proj_item_ids: Vec<I> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
pub type DroneAddCmdCtxFit = DroneAddCmdCtxFitGen<FitId, ItemId>;
pub type DroneAddCmdCtxFitBr = DroneAddCmdCtxFitGen<FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct DroneAddCmdCtxFitGen<F, I> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: DroneAddCmdGen<I>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> DroneAddCmdGen<I> {
    pub fn new(type_id: ItemTypeId, state: MinionState) -> Self {
        Self { type_id, state, .. }
    }
    pub fn with_mutation(mut self, mutation: AddMutation) -> Self {
        self.mutation = Some(mutation);
        self
    }
    pub fn with_npc_prop(mut self, npc_prop: NpcProp) -> Self {
        self.npc_prop = Some(npc_prop);
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.movement = Some(movement);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = I>) -> Self {
        self.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
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
impl DroneAddCmdBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<DroneAddCmd, BrResolveError> {
        Ok(DroneAddCmd {
            proj_item_ids: resps.resolve_item_ids(self.proj_item_ids)?,
            type_id: self.type_id,
            state: self.state,
            mutation: self.mutation,
            npc_prop: self.npc_prop,
            coordinates: self.coordinates,
            movement: self.movement,
            effect_modes: self.effect_modes,
        })
    }
}

impl DroneAddCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<DroneAddCmdCtxFit, BrResolveError> {
        Ok(DroneAddCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> DroneAddCmdGen<I> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        match self.proj_item_ids.is_empty() {
            true => CmdResidue::MutInfallible,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}
impl<F, I> DroneAddCmdCtxFitGen<F, I> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        match self.core.proj_item_ids.is_empty() {
            true => CmdResidue::MutFallibleClean,
            false => CmdResidue::MutFallibleDirty,
        }
    }
}

impl DroneAddCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, DroneAddError> {
        let mut core_drone = core_fit.add_drone(self.type_id, self.state, self.coordinates, self.movement);
        if let Some(mutation) = self.mutation.as_ref() {
            let mut core_mutation = core_drone.mutate(mutation.mutator_id).unwrap();
            mutation.apply_attrs(&mut core_mutation);
        }
        if let Some(npc_prop) = self.npc_prop {
            core_drone.set_npc_prop(Some(npc_prop))
        }
        self.effect_modes.apply(&mut core_drone);
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
