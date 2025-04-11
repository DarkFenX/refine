use crate::{
    cmd::{HCmdResp, change_item},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HSetCharacterCmd {
    type_id: rc::ItemTypeId,
    state: Option<bool>,
}
impl HSetCharacterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<rc::CharacterInfo, HExecError> {
        let core_character = match core_sol.set_fit_character(fit_id, self.type_id, self.state.unwrap_or(true)) {
            Ok(core_character) => core_character,
            Err(error) => {
                return Err(match error {
                    rc::err::SetFitCharacterError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        };
        Ok(core_character)
    }
}

#[derive(serde::Deserialize)]
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
    ) -> Result<HCmdResp, HExecError> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
            Self::ViaFitId(cmd) => cmd.execute(core_sol, fit_id),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeCharacterViaItemIdCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeCharacterCmd,
}
impl HChangeCharacterViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeCharacterViaFitIdCmd {
    #[serde(flatten)]
    item_cmd: change_item::HChangeCharacterCmd,
}
impl HChangeCharacterViaFitIdCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCmdResp, HExecError> {
        let item_id = match core_sol.get_fit_character_info(fit_id) {
            Ok(core_character) => match core_character {
                Some(core_character) => core_character.id,
                None => return Err(HExecError::FitCharacterNotFound(*fit_id)),
            },
            Err(error) => {
                return Err(match error {
                    rc::err::GetFitCharacterInfoError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        };
        self.item_cmd.execute(core_sol, &item_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HRemoveCharacterCmd {}
impl HRemoveCharacterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<(), HExecError> {
        if let Err(error) = core_sol.remove_fit_character(fit_id) {
            return Err(match error {
                rc::err::RemoveFitCharacterError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                rc::err::RemoveFitCharacterError::FitHasNoCharacter(e) => HExecError::FitItemKindNotFound(e),
            });
        };
        Ok(())
    }
}
