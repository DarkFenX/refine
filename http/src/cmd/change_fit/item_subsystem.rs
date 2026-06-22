use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp, change_item,
        shared::{HEffectModeMap, get_primary_fit},
    },
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HAddSubsystemCmd {
    type_id: i32,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HAddSubsystemCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        let mut core_subsystem = core_fit.add_subsystem(core_type_id);
        if let Some(state) = self.state {
            core_subsystem.set_state(state);
        }
        if let Some(effect_modes) = self.effect_modes.as_ref() {
            effect_modes.apply(&mut core_subsystem);
        }
        Ok(HItemIdsResp::from_core_subsystem(core_subsystem))
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeSubsystemCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeSubsystemCmd,
}
impl HChangeSubsystemCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
