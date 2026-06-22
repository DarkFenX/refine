use rc::ItemCommon;
use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp, change_item,
        shared::{HEffectModeMap, get_primary_fit},
    },
    util::HExecError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Setting
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
pub(crate) struct HSetStanceCmd {
    type_id: i32,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HSetStanceCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        let mut core_stance = core_fit.set_stance(core_type_id);
        if let Some(state) = self.state {
            core_stance.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_stance);
        }
        Ok(HItemIdsResp::from_core_stance(core_stance))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Changing
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeStanceCmd {
    ViaItemId(HChangeStanceViaItemIdCmd),
    ViaFitId(HChangeStanceViaFitIdCmd),
}
impl HChangeStanceCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
            Self::ViaFitId(cmd) => cmd.execute(core_sol, fit_id),
        }
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeStanceViaItemIdCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeStanceCmd,
}
impl HChangeStanceViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}

#[derive(Deserialize)]
pub(crate) struct HChangeStanceViaFitIdCmd {
    #[serde(flatten)]
    item_cmd: change_item::HChangeStanceCmd,
}
impl HChangeStanceViaFitIdCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let core_fit = get_primary_fit(core_sol, fit_id)?;
        let stance_item_id = match core_fit.get_stance() {
            Some(core_stance) => core_stance.get_item_id(),
            None => return Err(HExecError::FitStanceNotFound(*fit_id)),
        };
        self.item_cmd.execute(core_sol, &stance_item_id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Removing
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
pub(crate) struct HUnsetStanceCmd;
impl HUnsetStanceCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<(), HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        if let Some(core_stance) = core_fit.get_stance_mut() {
            core_stance.remove();
        }
        Ok(())
    }
}
