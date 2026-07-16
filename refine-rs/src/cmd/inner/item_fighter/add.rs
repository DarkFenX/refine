use crate::{
    AddedItemIdsResp, CmdResps, Coordinates, CountNz, FitId, FitIdBackref, ItemId, ItemIdBackref, ItemTypeId,
    MinionState, Movement, RearmMinion,
    cmd::shared::{Abilities, EffectModes},
    err::BackrefRenderError,
};

// Commands with full context
pub(in crate::cmd) struct ICmdFighterAddFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdFighterAddICtxBIds,
}
pub(crate) struct ICmdFighterAddFCtxRIds {
    pub(in crate::cmd) fit_id: FitId,
    pub(in crate::cmd) ictx_cmd: ICmdFighterAddICtxRIds,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdFighterAddICtxBIds {
    pub(in crate::cmd) shared: ICmdFighterAddShared,
    pub(in crate::cmd) proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
}
pub(crate) struct ICmdFighterAddICtxRIds {
    pub(in crate::cmd) shared: ICmdFighterAddShared,
    pub(in crate::cmd) proj_item_ids: Vec<ItemId> = Vec::new(),
}
pub(in crate::cmd) struct ICmdFighterAddShared {
    pub(in crate::cmd) type_id: ItemTypeId,
    pub(in crate::cmd) state: MinionState,
    pub(in crate::cmd) count: Option<CountNz> = None,
    pub(in crate::cmd) abilities: Abilities = Abilities::new(),
    pub(in crate::cmd) rearm_minion: Option<RearmMinion> = None,
    pub(in crate::cmd) coordinates: Option<Coordinates> = None,
    pub(in crate::cmd) movement: Option<Movement> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFighterAddFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFighterAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdFighterAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdFighterAddICtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFighterAddICtxRIds, BackrefRenderError> {
        Ok(ICmdFighterAddICtxRIds {
            shared: self.shared,
            proj_item_ids: resps.render_item_ids(self.proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFighterAddFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddFighterError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddFighterError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
    #[error("{0}")]
    AddFailed(#[from] FitAddFighterError),
}

impl ICmdFighterAddICtxRIds {
    pub(in crate::cmd) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, FitAddFighterError> {
        let mut core_fighter = core_fit.add_fighter(
            self.shared.type_id,
            self.shared.state,
            self.shared.coordinates,
            self.shared.movement,
        );
        if let Some(count) = self.shared.count {
            core_fighter.set_count_override(Some(count));
        }
        self.shared.abilities.apply(&mut core_fighter);
        if let Some(rearm_minion) = self.shared.rearm_minion {
            core_fighter.set_rearm_minion(Some(rearm_minion));
        }
        self.shared.effect_modes.apply(&mut core_fighter);
        for projectee_item_id in self.proj_item_ids.iter() {
            core_fighter.add_proj(projectee_item_id)?;
        }
        Ok(AddedItemIdsResp::from_core_fighter(core_fighter))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitAddFighterError {
    #[error("failed to add projection: {0}")]
    ProjAddFailed(#[from] rc::err::AddProjError),
}
