use serde::Deserialize;

use crate::{
    cmd::{HItemIdsResp, shared::HEffectModeMap},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HChangeStanceCmd {
    type_id: Option<i32>,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeStanceCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_stance = core_sol.get_stance_mut(item_id).map_err(|error| match error {
            rc::err::GetStanceError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetStanceError::ItemIsNotStance(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_stance.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_stance.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_stance);
        }
        Ok(HItemIdsResp::from_core_stance(core_stance))
    }
}
