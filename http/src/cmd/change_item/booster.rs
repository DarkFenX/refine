use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, HSideEffectMap, apply_effect_modes, apply_side_effects},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeBoosterCmd {
    #[serde(default)]
    type_id: Option<rc::ItemTypeId>,
    #[serde(default)]
    state: Option<bool>,
    #[serde(default)]
    side_effects: Option<HSideEffectMap>,
    #[serde(default)]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeBoosterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_booster = core_sol.get_booster_mut(item_id).map_err(|error| match error {
            rc::err::GetBoosterError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetBoosterError::ItemIsNotBooster(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            core_booster.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_booster.set_state(state);
        }
        apply_side_effects(&mut core_booster, &self.side_effects);
        apply_effect_modes(&mut core_booster, &self.effect_modes);
        Ok(core_booster.into())
    }
}
