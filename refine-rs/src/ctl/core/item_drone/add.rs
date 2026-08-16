use crate::{
    AddMutation, AddedItemIdsResp, Coordinates, CtlCmdResps, FitId, FitIdBr, ItemId, ItemIdBr, ItemTypeId, MinionState,
    Movement, NpcProp, ctl::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdDroneAddFCtxBIds {
    pub(in crate::ctl) fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdDroneAddICtxBIds,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdDroneAddFCtxRIds {
    pub(in crate::ctl) fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdDroneAddICtxRIds,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdDroneAddICtxBIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) shared: ICmdDroneAddShared,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) proj_item_ids: Vec<ItemIdBr> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdDroneAddICtxRIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) shared: ICmdDroneAddShared,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) proj_item_ids: Vec<ItemId> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdDroneAddShared {
    pub(in crate::ctl) type_id: ItemTypeId,
    pub(in crate::ctl) state: MinionState,
    pub(in crate::ctl) mutation: Option<AddMutation> = None,
    pub(in crate::ctl) npc_prop: Option<NpcProp> = None,
    pub(in crate::ctl) coordinates: Option<Coordinates> = None,
    pub(in crate::ctl) movement: Option<Movement> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdDroneAddFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdDroneAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdDroneAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdDroneAddICtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdDroneAddICtxRIds, BackrefRenderError> {
        Ok(ICmdDroneAddICtxRIds {
            shared: self.shared,
            proj_item_ids: resps.render_item_ids(self.proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdDroneAddFCtxRIds {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddDroneError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddDroneError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
    #[error("failed to add projection")]
    ProjAdd(#[source] rc::err::AddProjError),
}
impl From<FitAddDroneError> for GetFitAddDroneError {
    fn from(err: FitAddDroneError) -> Self {
        match err {
            FitAddDroneError::ProjAdd(inner) => Self::ProjAdd(inner),
        }
    }
}

impl ICmdDroneAddICtxRIds {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, FitAddDroneError> {
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
pub enum FitAddDroneError {
    #[error("failed to add projection")]
    ProjAdd(#[from] rc::err::AddProjError),
}
