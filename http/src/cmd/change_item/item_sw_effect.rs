use serde::Deserialize;

use crate::{
    cmd::{HItemIdsResp, shared::HEffectModeMap},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HChangeSwEffectCmd {
    #[serde(default)]
    type_id: Option<i32>,
    #[serde(default)]
    state: Option<bool>,
    #[serde(default)]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeSwEffectCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_sw_effect = core_sol.get_sw_effect_mut(item_id).map_err(|error| match error {
            rc::err::GetSwEffectError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetSwEffectError::ItemIsNotSwEffect(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_sw_effect.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_sw_effect.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_sw_effect);
        }
        Ok(HItemIdsResp::from_core_sw_effect(core_sw_effect))
    }
}
