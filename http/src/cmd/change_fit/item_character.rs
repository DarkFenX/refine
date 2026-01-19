use rc::ItemCommon;
use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{HItemIdsResp, change_item, shared::get_primary_fit},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HSetCharacterCmd {
    type_id: i32,
    state: Option<bool>,
}
impl HSetCharacterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_character = core_fit.set_character(rc::ItemTypeId::from_i32(self.type_id));
        if let Some(state) = self.state {
            core_character.set_state(state);
        }
        Ok(HItemIdsResp::from_core_character(core_character))
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeCharacterCmd {
    ViaItemId(HChangeCharacterViaItemIdCmd),
    ViaFitId(HChangeCharacterViaFitIdCmd),
}
impl HChangeCharacterCmd {
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
pub(crate) struct HChangeCharacterViaItemIdCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeCharacterCmd,
}
impl HChangeCharacterViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}

#[derive(Deserialize)]
pub(crate) struct HChangeCharacterViaFitIdCmd {
    #[serde(flatten)]
    item_cmd: change_item::HChangeCharacterCmd,
}
impl HChangeCharacterViaFitIdCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let core_fit = get_primary_fit(core_sol, fit_id)?;
        let character_item_id = match core_fit.get_character() {
            Some(core_character) => core_character.get_item_id(),
            None => return Err(HExecError::FitCharacterNotFound(*fit_id)),
        };
        self.item_cmd.execute(core_sol, &character_item_id)
    }
}

#[derive(Deserialize)]
pub(crate) struct HRemoveCharacterCmd {}
impl HRemoveCharacterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<(), HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        if let Some(core_character) = core_fit.get_character_mut() {
            core_character.remove();
        }
        Ok(())
    }
}
