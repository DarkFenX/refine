use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp, change_item,
        shared::{HSideEffectMap, apply_side_effects, get_primary_fit},
    },
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HAddBoosterCmd {
    type_id: i32,
    state: Option<bool>,
    side_effects: Option<HSideEffectMap>,
}
impl HAddBoosterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_booster = core_fit.add_booster(rc::ItemTypeId::from_i32(self.type_id));
        if let Some(state) = self.state {
            core_booster.set_state(state);
        }
        apply_side_effects(&mut core_booster, &self.side_effects);
        Ok(HItemIdsResp::from_core_booster(core_booster))
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeBoosterCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeBoosterCmd,
}
impl HChangeBoosterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
