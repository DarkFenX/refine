use crate::{
    cmd::{
        HItemIdsResp, change_item,
        shared::{HAbilityMap, apply_abilities, get_primary_fit},
    },
    shared::HMinionState,
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddFighterCmd {
    type_id: rc::ItemTypeId,
    state: HMinionState,
    count: Option<rc::Count>,
    abilities: Option<HAbilityMap>,
}
impl HAddFighterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_fighter = core_fit.add_fighter(self.type_id, (&self.state).into());
        if let Some(count) = self.count {
            let fighter_count_override = rc::FighterCountOverride::new_checked(count)?;
            core_fighter.set_count_override(Some(fighter_count_override));
        }
        apply_abilities(&mut core_fighter, &self.abilities);
        Ok(core_fighter.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeFighterCmd,
}
impl HChangeFighterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
