use crate::{
    AddedItemIdsResp, Coordinates, CountNz, CtlCmdResps, FitId, FitIdBr, ItemId, ItemIdBr, ItemTypeId, MinionState,
    Movement, RearmMinion,
    ctl::shared::{Abilities, EffectModes},
    err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdFighterAddFCtxBIds {
    pub(in crate::ctl) fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdFighterAddICtxBIds,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdFighterAddFCtxRIds {
    pub(in crate::ctl) fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdFighterAddICtxRIds,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdFighterAddICtxBIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) shared: ICmdFighterAddShared,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) proj_item_ids: Vec<ItemIdBr> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdFighterAddICtxRIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) shared: ICmdFighterAddShared,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) proj_item_ids: Vec<ItemId> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdFighterAddShared {
    pub(in crate::ctl) type_id: ItemTypeId,
    pub(in crate::ctl) state: MinionState,
    pub(in crate::ctl) count: Option<CountNz> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) abilities: Abilities = Abilities::new(),
    pub(in crate::ctl) rearm_minion: Option<RearmMinion> = None,
    pub(in crate::ctl) coordinates: Option<Coordinates> = None,
    pub(in crate::ctl) movement: Option<Movement> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFighterAddFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdFighterAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdFighterAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdFighterAddICtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdFighterAddICtxRIds, BackrefRenderError> {
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
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddFighterError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddFighterError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
    #[error("failed to add projection")]
    ProjAdd(#[source] rc::err::AddProjError),
}
impl From<FitAddFighterError> for GetFitAddFighterError {
    fn from(err: FitAddFighterError) -> Self {
        match err {
            FitAddFighterError::ProjAdd(inner) => Self::ProjAdd(inner),
        }
    }
}

impl ICmdFighterAddICtxRIds {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<AddedItemIdsResp, FitAddFighterError> {
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
    #[error("failed to add projection")]
    ProjAdd(#[from] rc::err::AddProjError),
}
