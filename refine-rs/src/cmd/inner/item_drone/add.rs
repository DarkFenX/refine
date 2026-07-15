use crate::cmd::shared::{
    AddMutation, AddedItemIdsResp, BackrefRenderError, CmdResps, EffectModes, FitIdBackref, ItemIdBackref,
};

// Commands with full context
pub(in crate::cmd) struct ICmdDroneAddFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdDroneAddICtxBIds,
}
pub(crate) struct ICmdDroneAddFCtxRIds {
    pub(in crate::cmd) fit_id: rc::FitId,
    pub(in crate::cmd) ictx_cmd: ICmdDroneAddICtxRIds,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdDroneAddICtxBIds {
    pub(in crate::cmd) shared: ICmdDroneAddShared,
    pub(in crate::cmd) proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
}
pub(crate) struct ICmdDroneAddICtxRIds {
    pub(in crate::cmd) shared: ICmdDroneAddShared,
    pub(in crate::cmd) proj_item_ids: Vec<rc::ItemId> = Vec::new(),
}
pub(in crate::cmd) struct ICmdDroneAddShared {
    pub(in crate::cmd) type_id: rc::ItemTypeId,
    pub(in crate::cmd) state: rc::MinionState,
    pub(in crate::cmd) mutation: Option<AddMutation> = None,
    pub(in crate::cmd) npc_prop: Option<rc::NpcProp> = None,
    pub(in crate::cmd) coordinates: Option<rc::Coordinates> = None,
    pub(in crate::cmd) movement: Option<rc::Movement> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdDroneAddFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdDroneAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdDroneAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdDroneAddICtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdDroneAddICtxRIds, BackrefRenderError> {
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
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddDroneError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddDroneError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
    #[error("{0}")]
    AddFailed(#[from] FitAddDroneError),
}

impl ICmdDroneAddICtxRIds {
    pub(in crate::cmd) fn execute(&self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, FitAddDroneError> {
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
    #[error("failed to add projection: {0}")]
    ProjAddFailed(#[from] rc::err::AddProjError),
}
