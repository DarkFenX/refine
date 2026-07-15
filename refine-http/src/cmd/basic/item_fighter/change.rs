use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::shared::{HAbilityMap, HChangedItemIdsResp, HCmdResps, HEffectModeMap, HItemIdBackref},
    err::HExecError,
    shared::{HCoordinates, HMinionState, HMovement, HRearmMinion},
    util::TriStateField,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HFighterChangeCmdFCtxBIds {
    item_id: HItemIdBackref,
    #[serde(flatten)]
    ictx_cmd: HFighterChangeCmdICtxBIds,
}
pub(crate) struct HFighterChangeCmdFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: HFighterChangeCmdICtxRIds,
}

// Commands with incomplete context
#[derive(Deserialize)]
struct HFighterChangeCmdICtxBIds {
    #[serde(flatten)]
    shared: HFighterChangeCmdShared,
    #[serde(default)]
    add_proj_item_ids: Vec<HItemIdBackref>,
    #[serde(default)]
    rm_proj_item_ids: Vec<HItemIdBackref>,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HFighterChangeCmdICtxRIds {
    #[serde(flatten)]
    shared: HFighterChangeCmdShared,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    add_proj_item_ids: Vec<rc::ItemId>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    rm_proj_item_ids: Vec<rc::ItemId>,
}
#[derive(Deserialize)]
struct HFighterChangeCmdShared {
    type_id: Option<i32>,
    state: Option<HMinionState>,
    #[serde(default)]
    count: TriStateField<u32>,
    abilities: Option<HAbilityMap>,
    #[serde(default)]
    rearm_minion: TriStateField<HRearmMinion>,
    coordinates: Option<HCoordinates>,
    movement: Option<HMovement>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFighterChangeCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HFighterChangeCmdFCtxRIds, HExecError> {
        Ok(HFighterChangeCmdFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl HFighterChangeCmdICtxBIds {
    fn render(self, resps: &HCmdResps) -> Result<HFighterChangeCmdICtxRIds, HExecError> {
        Ok(HFighterChangeCmdICtxRIds {
            shared: self.shared,
            add_proj_item_ids: resps.render_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.render_item_ids(self.rm_proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFighterChangeCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.item_id)
    }
}

impl HFighterChangeCmdICtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HChangedItemIdsResp, HExecError> {
        let mut core_fighter = core_sol.get_fighter_mut(item_id).map_err(|error| match error {
            rc::err::GetFighterError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetFighterError::ItemIsNotFighter(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.shared.type_id {
            core_fighter.set_type_id(rc::ItemTypeId::from_i32(type_id));
        }
        if let Some(state) = &self.shared.state {
            core_fighter.set_state(state.into_core());
        }
        match self.shared.count {
            TriStateField::Value(count) => {
                let fighter_count_override = rc::CountNz::from_u32_checked(count)?;
                core_fighter.set_count_override(Some(fighter_count_override));
            }
            TriStateField::None => {
                core_fighter.set_count_override(None);
            }
            TriStateField::Absent => (),
        }
        if let Some(h_abilities) = self.shared.abilities.as_ref() {
            h_abilities.apply(&mut core_fighter);
        }
        match self.shared.rearm_minion {
            TriStateField::Value(h_rearm_minion) => core_fighter.set_rearm_minion(Some(h_rearm_minion.into_core())),
            TriStateField::None => core_fighter.set_rearm_minion(None),
            TriStateField::Absent => (),
        }
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_fighter
                .get_proj_mut(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .remove();
        }
        if let Some(coordinates) = self.shared.coordinates {
            core_fighter.set_coordinates(coordinates.into_core());
        }
        if let Some(movement) = self.shared.movement {
            core_fighter.set_movement(movement.into_core());
        }
        for projectee_item_id in self.add_proj_item_ids.iter() {
            core_fighter.add_proj(projectee_item_id).map_err(|error| match error {
                rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
            })?;
        }
        if let Some(h_effect_modes) = self.shared.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_fighter);
        }
        Ok(HChangedItemIdsResp::default())
    }
}
