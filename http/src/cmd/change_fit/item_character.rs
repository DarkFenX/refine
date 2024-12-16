use crate::{
    cmd::{change_item, HCmdResp},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HSetCharacterCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HSetCharacterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> Result<rc::SolCharacterInfo, HExecError> {
        let core_character = match core_sol.set_fit_character(*fit_id, self.type_id, self.state.unwrap_or(true)) {
            Ok(core_character) => core_character,
            Err(error) => {
                return Err(match error {
                    rc::err::SetFitCharacterError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                })
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
        fit_id: &rc::SolFitId,
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
    item_id: rc::SolItemId,
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
        fit_id: &rc::SolFitId,
    ) -> Result<HCmdResp, HExecError> {
        let item_id = match core_sol.get_fit_character(fit_id) {
            Ok(core_character) => match core_character {
                Some(core_character) => core_character.id,
                None => return Err(HExecError::FitCharacterNotFound(*fit_id)),
            },
            Err(error) => {
                return Err(match error {
                    rc::err::GetFitCharacterError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                })
            }
        };
        self.item_cmd.execute(core_sol, &item_id)
    }
}
