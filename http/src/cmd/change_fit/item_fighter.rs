use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp, change_item,
        shared::{HAbilityMap, apply_abilities, get_primary_fit},
    },
    shared::{HCoordinates, HMinionState, HMovement},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HAddFighterCmd {
    type_id: i32,
    state: HMinionState,
    count: Option<u32>,
    abilities: Option<HAbilityMap>,
    coordinates: Option<HCoordinates>,
    movement: Option<HMovement>,
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
        apply_abilities(&mut core_fighter, &self.abilities);
        Ok(core_fighter.into())
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
