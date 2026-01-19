use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    shared::HServiceState,
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HChangeServiceCmd {
    #[serde(default)]
    type_id: Option<i32>,
    #[serde(default)]
    state: Option<HServiceState>,
    #[serde(default)]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeServiceCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_service = core_sol.get_service_mut(item_id).map_err(|error| match error {
            rc::err::GetServiceError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetServiceError::ItemIsNotService(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_service.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_service.set_state(state.into_core());
        }
        apply_effect_modes(&mut core_service, &self.effect_modes);
        Ok(HItemIdsResp::from_core_service(core_service))
    }
}
