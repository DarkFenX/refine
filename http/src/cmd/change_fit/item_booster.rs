use crate::{
    cmd::{
        change_item,
        shared::{apply_side_effects, HSideEffectMap},
        HCmdResp,
    },
    util::HExecResult,
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
    ) -> HExecResult<rc::SolBoosterInfo> {
        let info = core_sol.add_booster(*fit_id, self.type_id, self.state.unwrap_or(true))?;
        if self.side_effects.is_none() {
            return Ok(info);
        };
        apply_side_effects(core_sol, &info.id, &self.side_effects)?;
        let info = core_sol.get_booster_info(&info.id)?;
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
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
