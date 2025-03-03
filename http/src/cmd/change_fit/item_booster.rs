use crate::{
    cmd::{
        HCmdResp, change_item,
        shared::{HSideEffectMap, apply_side_effects},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HAddBoosterCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    side_effects: Option<HSideEffectMap>,
}
impl HAddBoosterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> Result<rc::SolBoosterInfo, HExecError> {
        let core_booster = match core_sol.add_booster(*fit_id, self.type_id, self.state.unwrap_or(true)) {
            Ok(core_booster) => core_booster,
            Err(error) => {
                return Err(match error {
                    rc::err::AddBoosterError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        };
        if self.side_effects.is_none() {
            return Ok(core_booster);
        };
        apply_side_effects(core_sol, &core_booster.id, &self.side_effects)?;
        let info = core_sol.get_booster(&core_booster.id).unwrap();
        Ok(info)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeBoosterCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeBoosterCmd,
}
impl HChangeBoosterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
