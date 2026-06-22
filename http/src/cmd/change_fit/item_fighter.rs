use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp, change_item,
        shared::{HAbilityMap, HEffectModeMap, get_primary_fit},
    },
    shared::{HCoordinates, HMinionState, HMovement, HRearmMinion},
    util::HExecError,
};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HAddFighterCmd {
    type_id: i32,
    state: HMinionState,
    count: Option<u32>,
    abilities: Option<HAbilityMap>,
    rearm_minion: Option<HRearmMinion>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    projs: Vec<rc::ItemId>,
    coordinates: Option<HCoordinates>,
    movement: Option<HMovement>,
    effect_modes: Option<HEffectModeMap>,
}
impl HAddFighterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_fighter = core_fit.add_fighter(
            rc::ItemTypeId::from_i32(self.type_id),
            self.state.into_core(),
            self.coordinates.map(|v| v.into_core()),
            self.movement.map(|v| v.into_core()),
        );
        if let Some(count) = self.count {
            let fighter_count_override = rc::FighterCount::from_u32_checked(count)?;
            core_fighter.set_count_override(Some(fighter_count_override));
        }
        if let Some(abilities) = self.abilities.as_ref() {
            abilities.apply(&mut core_fighter);
        }
        if let Some(h_rearm_minion) = self.rearm_minion {
            core_fighter.set_rearm_minion(Some(h_rearm_minion.into_core()));
        }
        for projectee_item_id in self.projs.iter() {
            core_fighter.add_proj(projectee_item_id).map_err(|error| match error {
                rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
            })?;
        }
        if let Some(effect_modes) = self.effect_modes.as_ref() {
            effect_modes.apply(&mut core_fighter);
        }
        Ok(HItemIdsResp::from_core_fighter(core_fighter))
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeFighterCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeFighterCmd,
}
impl HChangeFighterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
