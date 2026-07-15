use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::shared::{
        HAbilityMap, HCmdResps, HCreatedItemIdsResp, HEffectModeMap, HFitIdBackref, HItemIdBackref, get_primary_fit,
    },
    err::HExecError,
    shared::{HCoordinates, HMinionState, HMovement, HRearmMinion},
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HFighterAddCmdFCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HFighterAddCmdICtxBIds,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HFighterAddCmdFCtxRIds {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    ictx_cmd: HFighterAddCmdICtxRIds,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HFighterAddCmdICtxBIds {
    #[serde(flatten)]
    shared: HFighterAddCmdShared,
    #[serde(default)]
    proj_item_ids: Vec<HItemIdBackref>,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HFighterAddCmdICtxRIds {
    #[serde(flatten)]
    shared: HFighterAddCmdShared,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    proj_item_ids: Vec<rc::ItemId>,
}
#[derive(Deserialize)]
struct HFighterAddCmdShared {
    type_id: i32,
    state: HMinionState,
    count: Option<u32>,
    abilities: Option<HAbilityMap>,
    rearm_minion: Option<HRearmMinion>,
    coordinates: Option<HCoordinates>,
    movement: Option<HMovement>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFighterAddCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HFighterAddCmdFCtxRIds, HExecError> {
        Ok(HFighterAddCmdFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl HFighterAddCmdICtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HFighterAddCmdICtxRIds, HExecError> {
        Ok(HFighterAddCmdICtxRIds {
            shared: self.shared,
            proj_item_ids: resps.render_item_ids(self.proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFighterAddCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCreatedItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fit_id)
    }
}

impl HFighterAddCmdICtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCreatedItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_fighter = core_fit.add_fighter(
            rc::ItemTypeId::from_i32(self.shared.type_id),
            self.shared.state.into_core(),
            self.shared.coordinates.map(|v| v.into_core()),
            self.shared.movement.map(|v| v.into_core()),
        );
        if let Some(count) = self.shared.count {
            let fighter_count_override = rc::CountNz::from_u32_checked(count)?;
            core_fighter.set_count_override(Some(fighter_count_override));
        }
        if let Some(h_abilities) = self.shared.abilities.as_ref() {
            h_abilities.apply(&mut core_fighter);
        }
        if let Some(h_rearm_minion) = self.shared.rearm_minion {
            core_fighter.set_rearm_minion(Some(h_rearm_minion.into_core()));
        }
        for projectee_item_id in self.proj_item_ids.iter() {
            core_fighter.add_proj(projectee_item_id).map_err(|error| match error {
                rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
            })?;
        }
        if let Some(h_effect_modes) = self.shared.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_fighter);
        }
        Ok(HCreatedItemIdsResp::from_core_fighter(core_fighter))
    }
}
