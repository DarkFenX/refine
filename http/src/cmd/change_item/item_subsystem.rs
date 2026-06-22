use serde::Deserialize;

use crate::{
    cmd::{HItemIdsResp, shared::HEffectModeMap},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HChangeSubsystemCmd {
    type_id: Option<i32>,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeSubsystemCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_subsystem = core_sol.get_subsystem_mut(item_id).map_err(|error| match error {
            rc::err::GetSubsystemError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetSubsystemError::ItemIsNotSubsystem(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_subsystem.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_subsystem.set_state(state);
        }
        if let Some(effect_modes) = self.effect_modes.as_ref() {
            effect_modes.apply(&mut core_subsystem);
        }
        Ok(HItemIdsResp::from_core_subsystem(core_subsystem))
    }
}
